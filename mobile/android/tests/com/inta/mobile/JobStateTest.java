package com.inta.mobile;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public final class JobStateTest {
    private static void must(boolean condition, String message) {
        if (!condition) throw new AssertionError(message);
    }

    public static void main(String[] args) throws Exception {
        Path dir = Path.of(args[0]);
        Files.createDirectories(dir);
        Path file = dir.resolve("job.v1");
        JobStore store = new JobStore(file);
        JobTimer active = store.read().start(4200, 11).check(2, true);
        must(active.elapsed(7200, 11) == 3000, "same boot counts device sleep");
        store.write(active.checkpoint(8200, 11));
        JobTimer reopened = new JobStore(file).read();
        must(reopened.elapsed(12300, 11) == 8100 && reopened.steps == 4, "same-boot process recreation");
        Path otherPhone = Files.createDirectory(dir.resolveSibling(dir.getFileName() + "-other"));
        Path transferred = otherPhone.resolve("job.v1");
        Files.copy(file, transferred);
        JobTimer copied = new JobStore(transferred).read();
        must(copied.phase == JobTimer.Phase.PAUSED && copied.interrupted
            && copied.elapsed(63000, 11) == 4000, "another phone with the same boot count must not add time");
        JobTimer interrupted = reopened.reconcile(1, 12);
        must(interrupted.phase == JobTimer.Phase.PAUSED && interrupted.interrupted
            && interrupted.elapsed(99999, 12) == 4000, "next boot retains only last durable interval");
        must(reopened.reconcile(1, 11).interrupted, "impossible backwards elapsed time");
        must(interrupted.start(2, -1) == interrupted, "unverified boot cannot run timer");
        JobTimer paused = interrupted.start(100, 12).pause(600, 12);
        must(paused.elapsed(9000, 12) == 4500, "paused time must not accrue");
        JobTimer done = paused.start(1000, 12).complete(2500, 12);
        must(done.elapsed(9000, 12) == 6000 && done.steps == 4, "finish freezes time without inventing checklist entries");
        must(done.check(3, true) == done && done.start(10, 12) == done, "completed job cannot mutate");
        store.write(done);
        must(store.read().recordedMs == 6000 && store.read().phase == JobTimer.Phase.COMPLETE, "completed job survives atomic write and read");
        long count;
        try (var entries = Files.list(dir)) { count = entries.count(); }
        must(count == 2, "successful writes leave only job and local clock owner");
        byte[] exact = Files.readAllBytes(file);
        byte[] damaged = java.util.Arrays.copyOf(exact, exact.length + 1);
        Files.write(file, damaged);
        try { store.read(); throw new AssertionError("invalid record accepted"); }
        catch (IOException expected) { must(java.util.Arrays.equals(damaged, Files.readAllBytes(file)), "invalid record preserved"); }
        Path blocked = dir.resolve("blocked");
        Files.createDirectory(blocked);
        Path original = blocked.resolve("keep");
        Files.write(original, new byte[] { 42 });
        try { new JobStore(blocked).write(done); throw new AssertionError("invalid destination replaced"); }
        catch (IOException expected) { must(Files.readAllBytes(original)[0] == 42, "write failure preserves target"); }
        try (var entries = Files.list(dir)) {
            must(entries.noneMatch(path -> path.getFileName().toString().startsWith("job-stage-")), "failed write cleans only staging");
        }
        must(new JobTimer(Long.MAX_VALUE, 0, 12, 0, JobTimer.Phase.RUNNING, false)
            .elapsed(Long.MAX_VALUE, 12) == Long.MAX_VALUE, "long job duration must not overflow");
        Path planFile = dir.resolve("repair-draft");
        RepairDraft plan = RepairDraft.read(planFile)
            .append(new RepairDraft.Step(RepairDraft.Kind.SCAN, "Scan after reconnect"))
            .append(new RepairDraft.Step(RepairDraft.Kind.FIX, "Review the printer queue"));
        plan.move(1, -1).save(planFile);
        RepairDraft reopenedPlan = RepairDraft.read(planFile);
        must(reopenedPlan.steps.get(0).kind == RepairDraft.Kind.FIX && plan.steps.get(0).kind == RepairDraft.Kind.SCAN,
            "durable reordering must not mutate the original draft");
        reopenedPlan.remove(0).save(planFile);
        must(RepairDraft.read(planFile).steps.size() == 1, "step removal survives reopening");
        try { new RepairDraft.Step(RepairDraft.Kind.FIX, "hidden\nextra"); throw new AssertionError("multiline draft label accepted"); }
        catch (IllegalArgumentException expected) { }
        System.out.println("Offline job, repair draft and atomic storage acceptance passed.");
    }
}
