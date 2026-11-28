package com.inta.mobile;

import java.io.DataInputStream;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.UUID;

/** One atomically replaced private index. The first job keeps its original file locations. */
final class JobBook {
    static final String ORIGINAL = "original";
    private static final int MAGIC = 0x494a4201;
    final Map<String, String> jobs = new LinkedHashMap<>();
    private final Path root;
    String current;

    private JobBook(Path root) { this.root = root; }

    static JobBook read(Path root) throws IOException {
        JobBook book = new JobBook(root);
        DataInputStream stream;
        try { stream = new DataInputStream(Files.newInputStream(root.resolve("job-book.v1"))); }
        catch (NoSuchFileException firstUse) {
            book.current = ORIGINAL;
            book.jobs.put(ORIGINAL, "Repair job");
            return book;
        }
        try (DataInputStream in = stream) {
            if (in.readInt() != MAGIC) throw new IOException("Unrecognized job index");
            book.current = in.readUTF();
            int count = in.readInt();
            for (int n = 0; n < count; n++) {
                String id = in.readUTF(), name = in.readUTF();
                if ((!ORIGINAL.equals(id) && !id.matches("[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}"))
                    || !validName(name) || book.jobs.put(id, name) != null) throw new IOException("Invalid job entry");
            }
            if (!book.jobs.containsKey(ORIGINAL) || !book.jobs.containsKey(book.current) || in.read() != -1)
                throw new IOException("Incomplete job index");
        }
        return book;
    }

    private static boolean validName(String name) {
        return !name.trim().isEmpty() && name.length() <= 80 && name.codePoints().noneMatch(Character::isISOControl);
    }

    Path directory() {
        return ORIGINAL.equals(current) ? root : root.resolve("repair-jobs").resolve(current);
    }

    static synchronized void create(Path root, String name) throws IOException {
        name = name.trim();
        if (!validName(name)) throw new IOException("Enter a job name of 1–80 characters.");
        JobBook book = read(root);
        for (String existing : book.jobs.values())
            if (existing.equalsIgnoreCase(name)) throw new IOException("A job with that name already exists.");
        String id = UUID.randomUUID().toString();
        Files.createDirectories(root.resolve("repair-jobs"));
        Files.createDirectory(root.resolve("repair-jobs").resolve(id));
        book.jobs.put(id, name);
        book.current = id;
        book.persist();
    }

    static synchronized void choose(Path root, String id) throws IOException {
        JobBook book = read(root);
        if (!book.jobs.containsKey(id)) throw new IOException("The selected job is unavailable.");
        book.current = id;
        if (!Files.isDirectory(book.directory())) throw new IOException("Job storage is unavailable.");
        book.persist();
    }

    private void persist() throws IOException {
        JobStore.publish(root.resolve("job-book.v1"), out -> {
            out.writeInt(MAGIC);
            out.writeUTF(current);
            out.writeInt(jobs.size());
            for (Map.Entry<String, String> job : jobs.entrySet()) {
                out.writeUTF(job.getKey());
                out.writeUTF(job.getValue());
            }
        });
    }
}
