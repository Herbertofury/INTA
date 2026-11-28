using System.Globalization;
using System.Security.Cryptography;
using System.Text;

namespace Inta.RepairQueue;

public sealed record ApprovalStatement(string TenantId, string SigningKeyId, string TechnicianId,
    string DeviceId, string PairingEpoch, string PlanSha256, long ValidUntilUnixSeconds)
{
    private static bool IsSha256(string value)
    {
        if (value.Length != 64) return false;
        foreach (char c in value)
            if (c is not (>= '0' and <= '9') and not (>= 'a' and <= 'f')) return false;
        return true;
    }

    public byte[] CanonicalBytes()
    {
        if (!RepairPlan.ValidIdentifier(TenantId) || !RepairPlan.ValidIdentifier(SigningKeyId)
            || !RepairPlan.ValidIdentifier(TechnicianId) || !RepairPlan.ValidIdentifier(DeviceId)
            || !RepairPlan.ValidIdentifier(PairingEpoch) || !IsSha256(PlanSha256) || ValidUntilUnixSeconds <= 0)
            throw new ArgumentException("Invalid signed approval statement");
        return Encoding.UTF8.GetBytes(string.Join('\n', "inta-reconnect-approval:v1", TenantId, SigningKeyId,
            TechnicianId, DeviceId, PairingEpoch, PlanSha256,
            ValidUntilUnixSeconds.ToString(CultureInfo.InvariantCulture)) + "\n");
    }
}

/// Trust inputs come from existing authenticated enrollment, never from a submitted plan.
public sealed class SignedPlanApprovalVerifier
{
    private readonly string tenantId;
    private readonly string keyId;
    private readonly byte[] pinnedSubjectPublicKeyInfo;

    public SignedPlanApprovalVerifier(string tenantId, string keyId, ReadOnlySpan<byte> publicKey)
    {
        if (!RepairPlan.ValidIdentifier(tenantId) || !RepairPlan.ValidIdentifier(keyId) || publicKey.Length is < 64 or > 128)
            throw new ArgumentException("Invalid approval verifier pin");
        this.tenantId = tenantId;
        this.keyId = keyId;
        pinnedSubjectPublicKeyInfo = publicKey.ToArray();
        using ECDsa _ = ReadPinnedKey();
    }

    public VerifiedPlanApproval Verify(RepairPlan plan, ApprovalStatement statement, ReadOnlySpan<byte> signature,
        DateTimeOffset now)
    {
        if (signature.Length != 64 || statement.TenantId != tenantId || statement.SigningKeyId != keyId
            || statement.DeviceId != plan.Device.DeviceId || statement.PairingEpoch != plan.Device.PairingEpoch
            || statement.PlanSha256 != plan.Digest || statement.ValidUntilUnixSeconds <= now.ToUnixTimeSeconds()
            || statement.ValidUntilUnixSeconds > plan.ExpiresAt.ToUnixTimeSeconds())
            throw new ArgumentException("Approval does not authorize this exact current plan");
        byte[] message = statement.CanonicalBytes();
        using ECDsa key = ReadPinnedKey();
        if (!key.VerifyData(message, signature, HashAlgorithmName.SHA256, DSASignatureFormat.IeeeP1363FixedFieldConcatenation))
            throw new CryptographicException("Plan approval signature did not verify");
        return new VerifiedPlanApproval(plan, statement.TechnicianId, statement.ValidUntilUnixSeconds, SHA256.HashData(message));
    }

    private ECDsa ReadPinnedKey()
    {
        ECDsa key = ECDsa.Create();
        try
        {
            key.ImportSubjectPublicKeyInfo(pinnedSubjectPublicKeyInfo, out int consumed);
            if (consumed != pinnedSubjectPublicKeyInfo.Length
                || key.ExportParameters(false).Curve.Oid.Value != "1.2.840.10045.3.1.7")
                throw new CryptographicException("Approval key must be an exact named P-256 SPKI key");
            return key;
        }
        catch { key.Dispose(); throw; }
    }

    // Only this verifier constructs this proof. The host still authenticates the reconnecting peer.
    public sealed class VerifiedPlanApproval
    {
        internal VerifiedPlanApproval(RepairPlan plan, string technician, long expiry, byte[] journalBinding)
        {
            Plan = plan;
            Proof = new PlanApproval(technician, plan.Device, plan.Digest);
            ExpiresAt = DateTimeOffset.FromUnixTimeSeconds(expiry);
            JournalBinding = Convert.ToHexStringLower(journalBinding);
        }
        public RepairPlan Plan { get; }
        internal PlanApproval Proof { get; }
        internal string JournalBinding { get; }
        public DateTimeOffset ExpiresAt { get; }
    }
}
