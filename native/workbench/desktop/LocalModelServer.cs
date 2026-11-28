using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private OperationJob modelServerJob;
        private Process modelServer;
        private string ownedModelDigest;

        [DllImport("iphlpapi.dll", SetLastError = false)]
        private static extern uint GetExtendedTcpTable(IntPtr table, ref int size, bool order, int family, int type, uint reserved);

        // MIB_TCPTABLE_OWNER_PID. Match IPv4 loopback and network-byte-order
        // port; confirm the listener belongs to the specific process handle we started.
        private static bool OwnsModelListener(int pid)
        {
            int size = 0;
            if (GetExtendedTcpTable(IntPtr.Zero, ref size, false, 2, 3, 0) != 122 || size < 4 || size > 8 * 1024 * 1024) return false;
            IntPtr buffer = Marshal.AllocHGlobal(size);
            try {
                if (GetExtendedTcpTable(buffer, ref size, false, 2, 3, 0) != 0) return false;
                int rows = Marshal.ReadInt32(buffer);
                if (rows < 0 || rows > (size - 4) / 24) return false;
                int loopback = BitConverter.ToInt32(IPAddress.Loopback.GetAddressBytes(), 0);
                for (int i = 0; i < rows; i++) {
                    int row = 4 + i * 24;
                    if (Marshal.ReadInt32(buffer, row + 4) == loopback &&
                        (ushort)IPAddress.NetworkToHostOrder(Marshal.ReadInt16(buffer, row + 8)) == 11434 &&
                        Marshal.ReadInt32(buffer, row + 20) == pid) return true;
                }
                return false;
            } finally { Marshal.FreeHGlobal(buffer); }
        }

        private void ReleaseModelServer()
        {
            // Closing this window's job closes only processes attached by this instance.
            if (modelServerJob != null) { modelServerJob.Dispose(); modelServerJob = null; }
            if (modelServer != null) { modelServer.Dispose(); modelServer = null; }
            ownedModelDigest = null;
        }

        private void OfferStopModelServer(WrapPanel panel, Entry entry)
        {
            if (modelServer == null || modelServer.HasExited) return;
            Action(panel, "Stop my model server\u2026", delegate {
                if (MessageBox.Show(window, "Stop the local model server started by this Desktop window?", "Stop local model server",
                    MessageBoxButton.OKCancel, MessageBoxImage.Question, MessageBoxResult.Cancel) != MessageBoxResult.OK) return;
                ReleaseModelServer(); ReviewExplanation(entry);
            });
        }

        private async Task OfferModelServer(Entry entry, CancellationToken token)
        {
            try {
                var state = Json.Object(await core.Read(new[] { "assistant", "server-status" }, token));
                if (token.IsCancellationRequested || Object.Equals(Json.Get(state, "listening"), true)) return;
                if (Object.Equals(Json.Get(state, "configured"), true)) {
                    Note("Prepared model: " + Json.Text(state, "model") + ". The verified store is " + Json.Text(state, "store") + ".");
                    Action(detailActions, "Start model server\u2026", delegate { StartModelServer(entry); });
                    Status("Local model server is stopped.");
                }
                Action(detailActions, "Prepare model store\u2026", delegate { PrepareModelStore(entry); });
            } catch (OperationCanceledException) { }
            catch (Exception error) { if (!token.IsCancellationRequested) Note(error.GetBaseException().Message); }
        }

        private void PrepareModelStore(Entry entry)
        {
            string path = ChooseFolder("Select a local Ollama model store containing the reviewed Qwen2.5:0.5b model.");
            if (path == null) return;
            if (MessageBox.Show(window, "Verify the model manifest and every layer, then remember this store for this user?\n\n" + path,
                "Prepare local model store", MessageBoxButton.OKCancel, MessageBoxImage.Question, MessageBoxResult.Cancel) != MessageBoxResult.OK) return;
            RunOperation("Prepare local model store", new[] { "assistant", "prepare", path, "--approve" }, true, delegate { ReviewExplanation(entry); });
        }

        private static string OllamaExecutable()
        {
            foreach (var root in new[] { Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "Programs"),
                    Environment.GetFolderPath(Environment.SpecialFolder.ProgramFiles) }) {
                string path = Path.Combine(root, "Ollama", "ollama.exe");
                if (!File.Exists(path)) continue;
                bool linked = (File.GetAttributes(path) & FileAttributes.ReparsePoint) != 0;
                for (var dir = new DirectoryInfo(Path.GetDirectoryName(path)); !linked && dir != null; dir = dir.Parent)
                    linked = (dir.Attributes & FileAttributes.ReparsePoint) != 0;
                if (!linked && PythonRuntime.TrustedPublisher(path, "Ollama Inc.")) return path;
            }
            throw new InvalidOperationException("Install the Windows application signed by Ollama Inc. to start the local model server.");
        }

        private async void StartModelServer(Entry entry)
        {
            if (changingSetting) return;
            if (MessageBox.Show(window, "Start the signed Ollama executable with cloud disabled and the verified local model store? The server will listen only on 127.0.0.1 and stop when this window closes.",
                "Start local model server", MessageBoxButton.OKCancel, MessageBoxImage.Question, MessageBoxResult.Cancel) != MessageBoxResult.OK) return;
            Cancel(ref detailRequest); detailRequest = new CancellationTokenSource(); var token = detailRequest.Token;
            LockNavigation(true); Message("Start local model server", "Checking the local endpoint and verifying all model files\u2026");
            string lastError = null;
            try {
                var plan = Json.Object(await core.Read(new[] { "assistant", "server-plan" }, token));
                if (Json.Text(plan, "endpoint") != "http://127.0.0.1:11434" || Json.Text(plan, "model") != "qwen2.5:0.5b")
                    throw new InvalidDataException("The core returned an unexpected server plan.");
                string store = Json.Text(plan, "store"), path = await Task.Run((Func<string>)OllamaExecutable);
                string state = Directory.CreateDirectory(Path.Combine(Path.GetDirectoryName(ThemePath), "ollama-service")).FullName;
                string temp = Directory.CreateDirectory(Path.Combine(state, "temp")).FullName;
                var info = new ProcessStartInfo { FileName = path, Arguments = "serve", WorkingDirectory = Path.GetDirectoryName(path),
                    UseShellExecute = false, CreateNoWindow = true, RedirectStandardError = true, RedirectStandardOutput = true };
                info.EnvironmentVariables.Clear();
                foreach (string key in new[] { "SystemRoot","SystemDrive","WINDIR","ProgramFiles","ProgramW6432","ProgramFiles(x86)",
                    "ProgramData","NUMBER_OF_PROCESSORS","PROCESSOR_ARCHITECTURE" }) {
                    string value = Environment.GetEnvironmentVariable(key); if (!String.IsNullOrEmpty(value)) info.EnvironmentVariables[key] = value;
                }
                info.EnvironmentVariables["PATH"] = Path.GetDirectoryName(path) + ";" + Environment.SystemDirectory;
                info.EnvironmentVariables["USERPROFILE"] = state; info.EnvironmentVariables["LOCALAPPDATA"] = state;
                info.EnvironmentVariables["APPDATA"] = state; info.EnvironmentVariables["TEMP"] = temp; info.EnvironmentVariables["TMP"] = temp;
                info.EnvironmentVariables["OLLAMA_TMPDIR"] = temp; info.EnvironmentVariables["OLLAMA_MODELS"] = store;
                info.EnvironmentVariables["OLLAMA_HOST"] = "127.0.0.1:11434"; info.EnvironmentVariables["OLLAMA_NO_CLOUD"] = "1";
                if (modelServer != null && !modelServer.HasExited) throw new InvalidOperationException("This window already has a model server.");
                ReleaseModelServer();
                using (var image = new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.Read)) {
                    if (!PythonRuntime.TrustedPublisher(path, "Ollama Inc.")) throw new InvalidDataException("Ollama publisher verification failed.");
                    modelServerJob = OperationJob.Create();
                    modelServer = Process.Start(info);
                    modelServerJob.Attach(modelServer);
                    modelServer.ErrorDataReceived += delegate(object sender, DataReceivedEventArgs e) {
                        if (!String.IsNullOrWhiteSpace(e.Data)) Interlocked.Exchange(ref lastError, e.Data.Substring(0, Math.Min(400,e.Data.Length)));
                    };
                    modelServer.BeginErrorReadLine(); modelServer.BeginOutputReadLine();
                }
                var timer = Stopwatch.StartNew();
                while (!modelServer.HasExited && !OwnsModelListener(modelServer.Id) && timer.Elapsed < TimeSpan.FromSeconds(15))
                    await Task.Delay(160, token);
                if (modelServer.HasExited || !OwnsModelListener(modelServer.Id))
                    throw new InvalidOperationException("The signed server did not acquire the local Ollama endpoint. " + Interlocked.CompareExchange(ref lastError,null,null));
                Status("Waiting for the local model server to finish starting\u2026");
                Dictionary<string, object> inventory = null;
                Exception startupError = null;
                for (int attempt = 0; attempt < 4 && !modelServer.HasExited; attempt++) {
                    try { inventory = Json.Object(await core.Read(new[] { "assistant", "models" }, token)); break; }
                    catch (InvalidOperationException e) { startupError = e; }
                    await Task.Delay(500,token);
                }
                if (inventory == null) throw new InvalidOperationException("Local Ollama did not finish initialization. " +
                    (startupError == null ? "" : startupError.Message) + " " + Interlocked.CompareExchange(ref lastError,null,null));
                bool present = false;
                foreach (object row in Json.Array(Json.Get(inventory, "models"))) {
                    var model = Json.Object(row);
                    if (Json.Text(model, "name") == Json.Text(plan, "model") && Json.Text(model, "digest") == Json.Text(plan, "digest")) present = true;
                }
                if (!present) throw new InvalidOperationException("The server did not report the verified model digest.");
                ownedModelDigest = Json.Text(plan, "digest");
                ReviewExplanation(entry);
            } catch (Exception error) {
                ReleaseModelServer();
                Message("Model server unavailable", error.GetBaseException().Message);
                Action(detailActions, "Review local explanation", delegate { ReviewExplanation(entry); }); AddBack();
                Status("Local model server could not start. " + error.GetBaseException().Message);
            } finally { LockNavigation(false); }
        }
    }
}
