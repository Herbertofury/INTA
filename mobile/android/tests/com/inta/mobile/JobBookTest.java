package com.inta.mobile;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;

public final class JobBookTest {
    public static void main(String[] args) throws Exception {
        Path root = Files.createDirectory(Path.of(args[0]));
        var legacy = new JobStore(root.resolve("offline-job.v2"));
        legacy.write(JobTimer.empty().check(0, true));
        byte[] previous = Files.readAllBytes(root.resolve("offline-job.v2"));
        check(JobBook.read(root).directory().equals(root));
        JobBook.create(root, "Work order 812");
        JobBook first = JobBook.read(root);
        check(first.jobs.size() == 2 && !first.directory().equals(root));
        var temporary = new JobStore(first.directory().resolve("offline-job.v2"));
        check(temporary.read().steps == 0);
        temporary.write(JobTimer.empty().check(1, true));
        JobBook.create(root, "Work order 813");
        check(JobBook.read(root).jobs.size() == 3);
        try { JobBook.create(root, "work order 812"); throw new AssertionError(); } catch (IOException expected) { }
        try { JobBook.choose(root, "../other"); throw new AssertionError(); } catch (IOException expected) { }
        JobBook.choose(root, first.current);
        check(new JobStore(JobBook.read(root).directory().resolve("offline-job.v2")).read().steps == 2);
        JobBook.choose(root, JobBook.ORIGINAL);
        check(Arrays.equals(previous, Files.readAllBytes(root.resolve("offline-job.v2"))) && legacy.read().steps == 1);
        byte[] broken = {0, 0};
        Files.write(root.resolve("job-book.v1"), broken);
        try { JobBook.create(root, "new"); throw new AssertionError(); } catch (IOException expected) { }
        check(Arrays.equals(broken, Files.readAllBytes(root.resolve("job-book.v1"))));
    }
    private static void check(boolean result) { if (!result) throw new AssertionError(); }
}
