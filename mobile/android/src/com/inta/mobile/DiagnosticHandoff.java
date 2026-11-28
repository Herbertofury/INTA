package com.inta.mobile;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.security.GeneralSecurityException;
import java.security.MessageDigest;
import java.security.SecureRandom;
import java.util.Arrays;
import javax.crypto.Cipher;
import javax.crypto.spec.GCMParameterSpec;
import javax.crypto.spec.SecretKeySpec;

/** Attended, one-use, offline diagnostic reader. This class has no Android runtime dependencies. */
final class DiagnosticHandoff {
    static final int SIZE = 87;
    static final int SECONDS = 600;
    private static final byte[] MAGIC = "INTADG01".getBytes(StandardCharsets.US_ASCII);
    private static final byte[] DOMAIN = "INTA-DIAGNOSTIC-SESSION-1".getBytes(StandardCharsets.US_ASCII);
    private final byte[] secret = new byte[16];
    private final long issued, started;
    private boolean closed;

    DiagnosticHandoff(long unixSeconds, long elapsedMillis) {
        issued = unixSeconds;
        started = elapsedMillis;
        new SecureRandom().nextBytes(secret);
    }

    synchronized boolean active(long now, long elapsed) {
        if (now < issued - 30 || now >= issued + SECONDS || elapsed < started || elapsed - started >= SECONDS * 1000L) revoke();
        return !closed;
    }
    synchronized long expires() { return issued + SECONDS; }
    synchronized String code(long now, long elapsed) {
        if (!active(now, elapsed)) throw new IllegalStateException("Pairing expired or revoked.");
        char[] hex = "0123456789abcdef".toCharArray();
        StringBuilder code = new StringBuilder("INTA1.").append(issued).append('.');
        for (byte b : secret) code.append(hex[(b & 255) >>> 4]).append(hex[b & 15]);
        return code.toString();
    }
    synchronized void revoke() {
        Arrays.fill(secret, (byte) 0);
        closed = true;
    }
    synchronized Snapshot read(byte[] packet, long now, long elapsed) throws GeneralSecurityException {
        if (!active(now, elapsed)) throw new GeneralSecurityException("Pairing expired, revoked or already used.");
        if (packet.length != SIZE) throw new GeneralSecurityException("Invalid report size.");
        ByteBuffer envelope = ByteBuffer.wrap(packet);
        byte[] magic = new byte[8], id = new byte[16], nonce = new byte[12];
        envelope.get(magic);
        long sentIssue = envelope.getLong();
        envelope.get(id).get(nonce);
        MessageDigest hash = MessageDigest.getInstance("SHA-256");
        hash.update(DOMAIN);
        if (!Arrays.equals(MAGIC, magic) || issued != sentIssue || !MessageDigest.isEqual(id, Arrays.copyOf(hash.digest(secret), 16)))
            throw new GeneralSecurityException("The report does not match the active pairing.");
        Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
        cipher.init(Cipher.DECRYPT_MODE, new SecretKeySpec(secret, "AES"), new GCMParameterSpec(128, nonce));
        cipher.updateAAD(packet, 0, 44);
        byte[] decoded = cipher.doFinal(packet, 44, packet.length - 44);
        try {
            if (decoded.length != 27) throw new GeneralSecurityException("Invalid diagnostic fields.");
            ByteBuffer input = ByteBuffer.wrap(decoded);
            long captured = input.getLong(), total = input.getLong(), available = input.getLong();
            int load = input.get() & 255, ac = input.get() & 255, battery = input.get() & 255;
            if (captured < issued - 30 || captured >= expires() || captured > now + 30 || total <= 0
                    || available < 0 || available > total || load > 100 || (ac > 1 && ac != 255) || (battery > 100 && battery != 255))
                throw new GeneralSecurityException("Invalid diagnostic fields or capture time.");
            revoke();
            return new Snapshot(captured, total, available, load, ac, battery);
        } finally {
            Arrays.fill(decoded, (byte) 0);
        }
    }
    static final class Snapshot {
        final long captured, total, available;
        final int load, ac, battery;
        Snapshot(long captured, long total, long available, int load, int ac, int battery) {
            this.captured = captured; this.total = total; this.available = available;
            this.load = load; this.ac = ac; this.battery = battery;
        }
    }
}
