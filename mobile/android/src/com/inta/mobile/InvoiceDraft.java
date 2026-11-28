package com.inta.mobile;

import java.io.DataInputStream;
import java.io.IOException;
import java.math.BigDecimal;
import java.math.RoundingMode;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;

final class InvoiceDraft {
    private static final int MAGIC = 0x494E5601;
    private static final long MAX_MINUTES = 31 * 24 * 60;
    enum Profile { RESIDENTIAL, BUSINESS }
    final long recordedMs;
    final long minutes;
    final long rateCents;
    final Profile profile;

    private InvoiceDraft(long recordedMs, long minutes, long rateCents, Profile profile) {
        if (recordedMs < 0 || minutes < -1 || minutes > MAX_MINUTES || rateCents < 0
                || rateCents > 9999999 || profile == null) throw new IllegalArgumentException("Invalid billing draft");
        this.recordedMs = recordedMs;
        this.minutes = minutes;
        this.rateCents = rateCents;
        this.profile = profile;
    }

    static InvoiceDraft prepare(JobTimer job, String rate, String duration, Profile profile) {
        if (job.phase != JobTimer.Phase.COMPLETE) throw new IllegalArgumentException("Finish the job first");
        String hourly = rate.trim();
        String override = duration.trim();
        if (!hourly.matches("[0-9]{1,5}(\\.[0-9]{1,2})?"))
            throw new IllegalArgumentException("Enter a USD hourly rate, e.g. 95.00.");
        if (!override.isEmpty() && !override.matches("[0-9]{1,5}"))
            throw new IllegalArgumentException("Enter whole billable minutes or leave blank.");
        long minutes = override.isEmpty() ? -1 : Long.parseLong(override);
        if (minutes > MAX_MINUTES) throw new IllegalArgumentException("Enter at most 44,640 minutes.");
        return new InvoiceDraft(job.recordedMs, minutes,
                new BigDecimal(hourly).movePointRight(2).longValueExact(), profile);
    }

    String hourlyDollars() { return money(rateCents); }
    String subtotalDollars() {
        long duration = minutes < 0 ? recordedMs : minutes * 60000;
        // Multiplication cannot overflow for a long-running job; round the final
        // subtotal once to USD cents, not the elapsed duration or hourly rate.
        return BigDecimal.valueOf(duration).multiply(BigDecimal.valueOf(rateCents))
                .divide(BigDecimal.valueOf(3600000), 0, RoundingMode.HALF_UP)
                .movePointLeft(2).toPlainString();
    }

    private static String money(long cents) { return BigDecimal.valueOf(cents, 2).toPlainString(); }

    static InvoiceDraft read(Path path, JobTimer job) throws IOException {
        if (job.phase != JobTimer.Phase.COMPLETE) return null;
        try (DataInputStream in = new DataInputStream(Files.newInputStream(path))) {
            if (in.readInt() != MAGIC) throw new IOException("Unrecognized invoice draft");
            long time = in.readLong(), minutes = in.readLong(), rate = in.readLong();
            int profile = in.readUnsignedByte();
            if (in.read() != -1 || profile > 1 || time != job.recordedMs)
                throw new IOException("Billing draft does not match this completed job");
            try { return new InvoiceDraft(time, minutes, rate, profile == 0 ? Profile.RESIDENTIAL : Profile.BUSINESS); }
            catch (IllegalArgumentException malformed) { throw new IOException("Invalid invoice draft", malformed); }
        } catch (NoSuchFileException missing) {
            return null;
        }
    }

    void save(Path path) throws IOException {
        JobStore.publish(path, out -> {
            out.writeInt(MAGIC);
            out.writeLong(recordedMs);
            out.writeLong(minutes);
            out.writeLong(rateCents);
            out.writeByte(profile == Profile.RESIDENTIAL ? 0 : 1);
        });
    }
}
