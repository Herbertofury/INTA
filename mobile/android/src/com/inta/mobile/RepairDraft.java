package com.inta.mobile;

import java.io.DataInputStream;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/** An intentionally non-executable list edited before device selection and approval. */
final class RepairDraft {
    enum Kind {
        UNINSTALL("Remove an app"), FIX("Apply a repair"), SCRIPT("Use a reviewed PowerShell recipe"),
        SCAN("Run a malware scan"), OPTIMIZE("Apply an optimization");
        final String label;
        Kind(String label) { this.label = label; }
        @Override public String toString() { return label; }
    }

    static final int LIMIT = 24;
    static final class Step {
        final Kind kind;
        final String label;
        Step(Kind kind, String label) {
            String value = label.trim();
            if (kind == null || value.isEmpty() || value.length() > 120
                || value.codePoints().anyMatch(cp -> Character.isISOControl(cp) || Character.getType(cp) == Character.FORMAT))
                throw new IllegalArgumentException("Use a clear one-line step label.");
            this.kind = kind;
            this.label = value;
        }
    }

    final List<Step> steps;
    RepairDraft() { this(Collections.emptyList()); }
    private RepairDraft(List<Step> steps) { this.steps = Collections.unmodifiableList(new ArrayList<>(steps)); }

    RepairDraft append(Step value) {
        if (steps.size() >= LIMIT) throw new IllegalArgumentException("A draft can contain up to 24 steps.");
        List<Step> copy = new ArrayList<>(steps);
        copy.add(value);
        return new RepairDraft(copy);
    }

    RepairDraft remove(int index) {
        List<Step> copy = new ArrayList<>(steps);
        copy.remove(index);
        return new RepairDraft(copy);
    }

    RepairDraft move(int index, int direction) {
        if (direction != -1 && direction != 1) throw new IllegalArgumentException("Move one place at a time.");
        List<Step> copy = new ArrayList<>(steps);
        Collections.swap(copy, index, index + direction);
        return new RepairDraft(copy);
    }

    String exportText() {
        if (steps.isEmpty()) throw new IllegalStateException("The draft has no steps.");
        StringBuilder report = new StringBuilder(
            "INTA repair plan draft\nNot approved. Nothing in this document runs or grants access.\n\n");
        for (int i = 0; i < steps.size(); ++i) {
            Step step = steps.get(i);
            report.append(i + 1).append(". ").append(step.kind.label).append(" — ").append(step.label).append('\n');
        }
        return report.toString();
    }

    static RepairDraft read(Path path) throws IOException {
        try (DataInputStream input = new DataInputStream(Files.newInputStream(path))) {
            if (input.readInt() != 0x44520100) throw new IOException("Unsupported draft data");
            int length = input.readUnsignedByte();
            if (length > LIMIT) throw new IOException("Invalid draft count");
            List<Step> values = new ArrayList<>();
            for (int i = 0; i < length; ++i)
                values.add(new Step(Kind.valueOf(input.readUTF()), input.readUTF()));
            if (input.read() != -1) throw new IOException("Invalid trailing draft data");
            return new RepairDraft(values);
        } catch (NoSuchFileException absent) {
            return new RepairDraft();
        }
    }

    void save(Path path) throws IOException {
        JobStore.publish(path, output -> {
            output.writeInt(0x44520100);
            output.writeByte(steps.size());
            for (Step step : steps) {
                output.writeUTF(step.kind.name());
                output.writeUTF(step.label);
            }
        });
    }
}
