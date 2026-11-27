using System.Collections.Concurrent;
using System.Text.Json;
namespace INTA.LocalAgent;
public sealed class OperationStore
{
    private readonly ConcurrentDictionary<string, OperationRecord> records = new();
    private readonly ConcurrentDictionary<string, SemaphoreSlim> packageLocks = new();
    private readonly string filePath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "INTA", "agent", "operations.jsonl");
    private static readonly JsonSerializerOptions Json = new(JsonSerializerDefaults.Web);
    public OperationStore()
    {
        Directory.CreateDirectory(Path.GetDirectoryName(filePath)!);
        if (!File.Exists(filePath)) return;
        foreach (var line in File.ReadLines(filePath).TakeLast(500))
        {
            try { var record = JsonSerializer.Deserialize<OperationRecord>(line, Json); if (record is not null) records[record.Id] = record; }
            catch { }
        }
        foreach (var pair in records.ToArray())
            if (pair.Value.State is "queued" or "running") records[pair.Key] = pair.Value with { State = "interrupted", Error = "INTA agent restarted while this operation was active. Inspect the installer and logs before retrying." };
    }
    public IReadOnlyList<OperationRecord> Recent() => records.Values.OrderByDescending(x => x.StartedAt).Take(100).ToArray();
    public OperationRecord? Get(string id) => records.GetValueOrDefault(id);
    public SemaphoreSlim LockFor(string key) => packageLocks.GetOrAdd(key, _ => new SemaphoreSlim(1, 1));
    public async Task SaveAsync(OperationRecord record)
    {
        records[record.Id] = record;
        await File.AppendAllTextAsync(filePath, JsonSerializer.Serialize(record, Json) + Environment.NewLine);
    }
}
