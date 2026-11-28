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
        Path draftFile = dir.resolve("invoice-draft");
        JobTimer billable = new JobTimer(5400000L, 0, -1, 7, JobTimer.Phase.COMPLETE, false);
        must(InvoiceDraft.read(draftFile, billable) == null, "no guessed rate or automatic charge");
        InvoiceDraft invoice = InvoiceDraft.prepare(billable, "95.00", "", InvoiceDraft.Profile.BUSINESS);
        must(invoice.subtotalDollars().equals("142.50"), "recorded 90 minutes at $95");
        invoice.save(draftFile);
        must(InvoiceDraft.read(draftFile, billable).profile == InvoiceDraft.Profile.BUSINESS, "reopen service profile");
        InvoiceDraft adjusted = InvoiceDraft.prepare(billable, "95", "5", InvoiceDraft.Profile.RESIDENTIAL);
        must(adjusted.subtotalDollars().equals("7.92") && adjusted.recordedMs == billable.recordedMs,
            "manual override rounds only the subtotal and preserves original time");
        adjusted.save(draftFile);
        InvoiceDraft saved = InvoiceDraft.read(draftFile, billable);
        must(saved.minutes == 5 && saved.hourlyDollars().equals("95.00") && saved.subtotalDollars().equals("7.92"), "durable override");
        must(InvoiceDraft.prepare(billable, "95", "0", InvoiceDraft.Profile.RESIDENTIAL).subtotalDollars().equals("0.00"), "explicit zero is not blank");
        try { InvoiceDraft.read(draftFile, done); throw new AssertionError("another job accepted"); }
        catch (IOException expected) { must(InvoiceDraft.read(draftFile, billable).minutes == 5, "mismatched draft remains preserved"); }
        try { InvoiceDraft.prepare(active, "95", "", InvoiceDraft.Profile.BUSINESS); throw new AssertionError("unfinished job billed"); }
        catch (IllegalArgumentException expected) { }
        try { InvoiceDraft.prepare(billable, "1e3", "", InvoiceDraft.Profile.BUSINESS); throw new AssertionError("exponent accepted"); }
        catch (IllegalArgumentException expected) { }
        String[] labels = {"Scope", "Scan", "Diagnose", "Update", "Retest", "Review"};
        String report = JobHandoff.render(billable, saved, labels);
        must(report.contains("Recorded time: 01:30:00") && report.contains("[x] Diagnose")
            && report.contains("[ ] Update") && report.contains("Billable minutes: 5")
            && report.contains("Draft subtotal: 7.92"), "exact completed record and stored draft in handoff");
        must(!JobHandoff.render(billable, null, labels).contains("subtotal"), "no inferred billing");
        try { JobHandoff.render(active, null, labels); throw new AssertionError("unfinished job exported"); }
        catch (IllegalArgumentException expected) { }
        try { JobHandoff.render(done, saved, labels); throw new AssertionError("mismatched invoice exported"); }
        catch (IllegalArgumentException expected) { }
        System.out.println("Offline job, repair draft, invoice calculation, handoff and atomic storage acceptance passed.");
    }
}
