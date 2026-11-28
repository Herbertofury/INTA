using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Web.Script.Serialization;
using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;

namespace Inta.Desktop
{
    internal static class Json
    {
        internal static JavaScriptSerializer Serializer()
        {
            return new JavaScriptSerializer { MaxJsonLength = 8 * 1024 * 1024, RecursionLimit = 64 };
        }

        internal static Dictionary<string, object> Object(object value)
        {
            var map = value as Dictionary<string, object>;
            if (map == null) throw new InvalidDataException("The core returned an unexpected record format.");
            return map;
        }

        internal static object Get(Dictionary<string, object> map, string key)
        {
            object value;
            return map.TryGetValue(key, out value) ? value : null;
        }

        internal static string Text(Dictionary<string, object> map, string key)
        {
            return Convert.ToString(Get(map, key), System.Globalization.CultureInfo.InvariantCulture);
        }

        internal static IEnumerable<object> Array(object value)
        {
            var items = value as IList;
            if (items == null) throw new InvalidDataException("The core returned an unexpected list format.");
            return items.Cast<object>();
        }
    }

    internal sealed class CoreClient
    {
        private readonly string executable;

        internal CoreClient(string directory)
        {
            executable = Path.Combine(directory, "inta.exe");
        }

        // Windows CommandLineToArgvW quoting, including trailing slashes and internal quotes.
        internal static string Quote(string argument)
        {
            var text = new StringBuilder("\"");
            int slashes = 0;
            foreach (char c in argument)
            {
                if (c == '\\') { slashes++; continue; }
                if (c == '"') text.Append('\\', 2 * slashes + 1);
                else text.Append('\\', slashes);
                slashes = 0;
                text.Append(c);
            }
            return text.Append('\\', 2 * slashes).Append('"').ToString();
        }

        // These streams are drained concurrently to avoid a full stderr pipe blocking stdout.
        private static async Task<string> ReadBounded(StreamReader reader, int limit)
        {
            var text = new StringBuilder();
            var buffer = new char[8192];
            int count;
            bool exceeded = false;
            while ((count = await reader.ReadAsync(buffer, 0, buffer.Length).ConfigureAwait(false)) > 0)
            {
                if (count > limit - text.Length) exceeded = true;
                if (!exceeded) text.Append(buffer, 0, count);
            }
            if (exceeded) throw new InvalidDataException("Output exceeded the desktop display limit. Use the CLI for this report.");
            return text.ToString();
        }
        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool PeekNamedPipe(SafeFileHandle pipe, IntPtr data, uint length, IntPtr read, out uint available, IntPtr remaining);
        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool ReadFile(SafeFileHandle pipe, [Out] byte[] buffer, uint length, out uint count, IntPtr overlapped);

        private static void DrainAvailable(FileStream pipe, MemoryStream destination, int limit, byte[] buffer)
        {
            uint available;
            while (PeekNamedPipe(pipe.SafeFileHandle, IntPtr.Zero, 0, IntPtr.Zero, out available, IntPtr.Zero))
            {
                if (available == 0) return;
                uint amount;
                if (!ReadFile(pipe.SafeFileHandle, buffer, Math.Min(available, (uint)buffer.Length), out amount, IntPtr.Zero))
                    throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
                int read = (int)amount;
                if (read == 0) return;
                if (read > limit - destination.Length) throw new InvalidDataException("Launcher output exceeded the desktop limit.");
                destination.Write(buffer, 0, read);
            }
            int error = Marshal.GetLastWin32Error();
            if (error != 109 && error != 232) throw new System.ComponentModel.Win32Exception(error);
        }
        private static object ReadLauncher(Process process, CancellationToken token)
        {
            // An explicitly launched application may outlive the CLI and retain a pipe.
            var output = (FileStream)process.StandardOutput.BaseStream;
            var error = (FileStream)process.StandardError.BaseStream;
            using (var result = new MemoryStream())
            using (var diagnostic = new MemoryStream()) {
                var buffer = new byte[8192];
                bool exited;
                do {
                    token.ThrowIfCancellationRequested();
                    exited = process.WaitForExit(90);
                    DrainAvailable(output, result, 1024 * 1024, buffer);
                    DrainAvailable(error, diagnostic, 256 * 1024, buffer);
                } while (!exited);
                if (process.ExitCode != 0)
                    throw new InvalidOperationException(diagnostic.Length == 0 ? "The launcher failed." : Encoding.UTF8.GetString(diagnostic.ToArray()).Trim());
                return Json.Serializer().DeserializeObject(Encoding.UTF8.GetString(result.ToArray()));
            }
        }

        internal async Task<object> Read(string[] arguments, CancellationToken cancellation)
        {
            if (!File.Exists(executable)) throw new FileNotFoundException("The INTA core is missing. Repair this installation to restore inta.exe.");
            cancellation.ThrowIfCancellationRequested();
            bool launchGuest = arguments.Length >= 4 && arguments[0] == "vm" && arguments[1] == "start" && arguments.Contains("--approve");
            bool externalTool = arguments.Length >= 4 && arguments[0] == "tool" &&
                (arguments[1] == "install" || arguments[1] == "launch" || arguments[1] == "linux") && arguments.Contains("--approve");
            bool launcher = launchGuest || externalTool;
            using (var process = new Process())
            using (var job = launcher ? null : OperationJob.Create())
            using (var timeout = new CancellationTokenSource(TimeSpan.FromMinutes(externalTool ? 60 : launchGuest ? 15 : 4)))
            using (var stop = CancellationTokenSource.CreateLinkedTokenSource(timeout.Token, cancellation))
            {
                process.StartInfo = new ProcessStartInfo {
                    FileName = executable, Arguments = String.Join(" ", arguments.Select(Quote)),
                    WorkingDirectory = Path.GetDirectoryName(executable), UseShellExecute = false,
                    CreateNoWindow = true, RedirectStandardError = true, RedirectStandardOutput = true,
                    StandardOutputEncoding = Encoding.UTF8, StandardErrorEncoding = Encoding.UTF8
                };
                process.EnableRaisingEvents = true;
                var completion = new TaskCompletionSource<int>();
                process.Exited += delegate { completion.TrySetResult(0); };
                process.Start();
                if (job != null) job.Attach(process);
                using (stop.Token.Register(delegate {
                    if (job != null) job.Stop();
                    else {
                        try { if (!process.HasExited) process.Kill(); }
                        catch (InvalidOperationException) { }
                        catch (System.ComponentModel.Win32Exception) { }
                    }
                }))
                {
                    if (launcher) {
                        try { return await Task.Run(() => ReadLauncher(process, stop.Token)).ConfigureAwait(false); }
                        catch (OperationCanceledException) {
                            cancellation.ThrowIfCancellationRequested();
                            throw new TimeoutException(launchGuest ? "The virtual machine launcher timed out. Check Lab sessions for its recorded state." :
                                "The tool action timed out. Check installation or application state before retrying.");
                        }
                    }
                    var output = ReadBounded(process.StandardOutput, 8 * 1024 * 1024);
                    var errors = ReadBounded(process.StandardError, 256 * 1024);
                    await Task.WhenAll(completion.Task, output, errors).ConfigureAwait(false);
                    cancellation.ThrowIfCancellationRequested();
                    if (timeout.IsCancellationRequested) throw new TimeoutException("The operation exceeded the desktop time limit.");
                    if (process.ExitCode == 3 && arguments.Length > 0 && arguments[0] == "scenario")
                    {
                        var result = Json.Serializer().DeserializeObject(output.Result);
                        if (Json.Text(Json.Object(result), "status") == "stopped") return result;
                    }
                    if (process.ExitCode != 0)
                    {
                        string message = errors.Result.Trim();
                        throw new InvalidOperationException(String.IsNullOrWhiteSpace(message) ? "The INTA core exited with code " + process.ExitCode + "." : message);
                    }
                    return Json.Serializer().DeserializeObject(output.Result);
                }
            }
        }
    }
}
