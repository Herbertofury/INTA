using System.Diagnostics;
using System.Text;
namespace INTA.LocalAgent;
public sealed class ProcessRunner
{
    public async Task<(int ExitCode, string Output)> RunAsync(string file, IReadOnlyList<string> args, TimeSpan timeout, CancellationToken cancellationToken = default)
    {
        var start = new ProcessStartInfo(file) { UseShellExecute = false, RedirectStandardOutput = true, RedirectStandardError = true, CreateNoWindow = true };
        foreach (var arg in args) start.ArgumentList.Add(arg);
        using var process = new Process { StartInfo = start, EnableRaisingEvents = true };
        var output = new StringBuilder();
        process.OutputDataReceived += (_, e) => Append(output, e.Data);
        process.ErrorDataReceived += (_, e) => Append(output, e.Data);
        try { if (!process.Start()) throw new InvalidOperationException($"Could not start {file}."); }
        catch (Exception ex) when (ex is System.ComponentModel.Win32Exception or InvalidOperationException)
        { throw new InvalidOperationException($"Could not launch {file}. Check that it is installed and available on PATH; elevated operations may require restarting INTA as administrator/root.", ex); }
        process.BeginOutputReadLine(); process.BeginErrorReadLine();
        using var timeoutCts = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
        timeoutCts.CancelAfter(timeout);
        try { await process.WaitForExitAsync(timeoutCts.Token); }
        catch (OperationCanceledException)
        {
            // Keep observing the original process. Never terminate an installer because the UI stopped waiting.
            await process.WaitForExitAsync(CancellationToken.None);
        }
        return (process.ExitCode, output.ToString());
    }
    private static void Append(StringBuilder output, string? line)
    {
        if (line is null) return;
        lock (output) { if (output.Length < 64000) output.AppendLine(line.Length > 4000 ? line[..4000] : line); }
    }
}
