package com.inta.mobile;

import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.UUID;

final class JobStore {
    private static final int MAGIC = 0x494E5402;
    private final Path file;

    JobStore(Path file) { this.file = file; }

    private UUID clockOwner() throws IOException {
        Path idFile = file.resolveSibling("offline-clock-owner.v1");
        try (DataInputStream record = new DataInputStream(Files.newInputStream(idFile))) {
            UUID id = new UUID(record.readLong(), record.readLong());
            if (record.read() != -1) throw new IOException("Invalid clock owner");
            return id;
        } catch (NoSuchFileException missing) {
            UUID id = UUID.randomUUID();
            publish(idFile, output -> {
                output.writeLong(id.getMostSignificantBits());
                output.writeLong(id.getLeastSignificantBits());
            });
            return id;
        }
    }

    JobTimer read() throws IOException {
        final DataInputStream stream;
        try {
            stream = new DataInputStream(Files.newInputStream(file));
        } catch (NoSuchFileException missing) {
            return JobTimer.empty();
        }
        try (DataInputStream input = stream) {
            if (input.readInt() != MAGIC) throw new IOException("Unsupported job format");
            UUID anchor = new UUID(input.readLong(), input.readLong());
            long recorded = input.readLong();
            long started = input.readLong();
            int boot = input.readInt();
            int steps = input.readByte();
            int phaseCode = input.readUnsignedByte();
            int flag = input.readUnsignedByte();
            if (input.read() != -1 || flag > 1) throw new IOException("Invalid job length or flags");
            JobTimer.Phase phase;
            switch (phaseCode) {
                case 0: phase = JobTimer.Phase.PAUSED; break;
                case 1: phase = JobTimer.Phase.RUNNING; break;
                case 2: phase = JobTimer.Phase.COMPLETE; break;
                default: throw new IOException("Invalid job phase");
            }
            JobTimer state = new JobTimer(recorded, started, boot, steps, phase, flag == 1);
            return state.phase == JobTimer.Phase.RUNNING && !anchor.equals(clockOwner())
                ? state.detachClock() : state;
        }
    }

    void write(JobTimer value) throws IOException {
        UUID owner = clockOwner();
        publish(file, output -> {
            output.writeInt(MAGIC);
            output.writeLong(owner.getMostSignificantBits());
            output.writeLong(owner.getLeastSignificantBits());
            output.writeLong(value.recordedMs);
            output.writeLong(value.startedAt);
            output.writeInt(value.boot);
            output.writeByte(value.steps);
            switch (value.phase) {
                case PAUSED: output.writeByte(0); break;
                case RUNNING: output.writeByte(1); break;
                case COMPLETE: output.writeByte(2); break;
                default: throw new IOException("Unsupported job phase");
            }
            output.writeByte(value.interrupted ? 1 : 0);
        });
    }

    @FunctionalInterface
    interface Payload { void write(DataOutputStream output) throws IOException; }

    static void publish(Path target, Payload payload) throws IOException {
        Path staged = Files.createTempFile(target.getParent(), "job-stage-", ".tmp");
        try {
            try (FileOutputStream bytes = new FileOutputStream(staged.toFile());
                    DataOutputStream output = new DataOutputStream(bytes)) {
                payload.write(output);
                output.flush();
                bytes.getFD().sync();
            }
            Files.move(staged, target, StandardCopyOption.ATOMIC_MOVE, StandardCopyOption.REPLACE_EXISTING);
        } finally {
            Files.deleteIfExists(staged);
        }
    }
}
