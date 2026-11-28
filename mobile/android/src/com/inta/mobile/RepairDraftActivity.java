package com.inta.mobile;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.ActivityNotFoundException;
import android.content.Intent;
import android.net.Uri;
import android.graphics.Color;
import android.graphics.Typeface;
import android.os.Build;
import android.os.Bundle;
import android.text.InputFilter;
import android.view.WindowInsets;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.Spinner;
import android.widget.TextView;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;

public final class RepairDraftActivity extends Activity {
    private static final Executor DISK = Executors.newSingleThreadExecutor();
    private static final int WHITE = 0xfff5f7fa, DIM = 0xffa8b1c1, ACCENT = 0xff9af4d8;
    private LinearLayout rows;
    private TextView status;
    private EditText label;
    private Button add, export;
    private static final int CREATE_DRAFT = 8;
    private String pendingExport;
    private boolean writingExport;
    private Spinner kind;
    private Path destination;
    private RepairDraft draft;
    private boolean pending;

    @Override protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        ScrollView scroll = new ScrollView(this);
        OpticalField optics = new OpticalField(this);
        optics.addView(scroll);
        scroll.setFillViewport(true);
        scroll.setOnApplyWindowInsetsListener((view, insets) -> {
            if (Build.VERSION.SDK_INT >= 30) {
                var area = insets.getInsets(WindowInsets.Type.systemBars() | WindowInsets.Type.displayCutout());
                view.setPadding(area.left, area.top, area.right, area.bottom);
            }
            return insets;
        });
        LinearLayout body = column();
        int margin = dp(20);
        body.setPadding(margin, margin, margin, margin);
        scroll.addView(body);
        text(body, "INTA MOBILE  /  LOCAL DRAFT", 12, ACCENT, true);
        text(body, "Repair plan", 32, WHITE, true);
        text(body, "Put steps in order before picking and approving a PC. Nothing runs from this draft.", 15, DIM, false);
        optics.addAppearanceButton(body);
        kind = new Spinner(this);
        ArrayAdapter<RepairDraft.Kind> picker = new ArrayAdapter<>(this, android.R.layout.simple_spinner_item, RepairDraft.Kind.values());
        picker.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item);
        kind.setAdapter(picker);
        kind.setContentDescription("Repair step name");
        kind.setMinimumHeight(dp(48));
        body.addView(kind, new LinearLayout.LayoutParams(-1,-2));
        label = new EditText(this);
        label.setHint("Step label, not a command");
        label.setSingleLine(true);
        label.setFilters(new InputFilter[]{new InputFilter.LengthFilter(120)});
        body.addView(label, new LinearLayout.LayoutParams(-1,-2));
        add = button(body, "Add to draft", () -> {
            try { save(draft.append(new RepairDraft.Step((RepairDraft.Kind)kind.getSelectedItem(), label.getText().toString())), true); }
            catch (IllegalArgumentException invalid) { status.setText(invalid.getMessage()); }
        });
        status = text(body, "Opening saved draft…", 14, DIM, false);
        rows = column();
        body.addView(rows);
        export = button(body, "Review draft export", this::reviewExport);
        export.setEnabled(false);
        setContentView(optics);
        add.setEnabled(false);
        DISK.execute(() -> {
            try {
                Path local = JobBook.read(getNoBackupFilesDir().toPath()).directory().resolve("repair-plan-draft.v1");
                RepairDraft loaded = RepairDraft.read(local);
                runOnUiThread(() -> {
                    if (!isDestroyed()) {
                        destination = local;
                        draft = loaded;
                        render("Saved on this phone.");
                    }
                });
            } catch (IOException | IllegalArgumentException failure) {
                runOnUiThread(() -> { if (!isDestroyed()) status.setText("Could not open the draft. Its saved data was left unchanged."); });
            }
        });
    }

    private void save(RepairDraft next, boolean clearInput) {
        if (pending) return;
        pending = true;
        render("Saving draft…");
        DISK.execute(() -> {
            boolean saved;
            try { next.save(destination); saved = true; }
            catch (IOException problem) { saved = false; }
            final boolean ok = saved;
            runOnUiThread(() -> {
                if (isDestroyed()) return;
                if (ok) {
                    draft = next;
                    if (clearInput) label.setText("");
                }
                pending = false;
                render(ok ? "Saved on this phone." : "Draft could not be saved. Previous steps are unchanged.");
            });
        });
    }

    private void reviewExport() {
        if (draft == null || draft.steps.isEmpty() || pending || pendingExport != null || writingExport) return;
        String report = draft.exportText();
        new AlertDialog.Builder(this)
            .setTitle("Review this draft before saving")
            .setMessage(report + "\nSave only to a location appropriate for this job.")
            .setNegativeButton("Cancel", null)
            .setPositiveButton("Choose save location", (dialog, which) -> chooseDestination(report))
            .show();
    }

    @SuppressWarnings("deprecation")
    private void chooseDestination(String report) {
        pendingExport = report;
        updateExport();
        Intent create = new Intent(Intent.ACTION_CREATE_DOCUMENT).addCategory(Intent.CATEGORY_OPENABLE)
            .setType("text/plain").putExtra(Intent.EXTRA_TITLE, "INTA-repair-draft.txt");
        try { startActivityForResult(create, CREATE_DRAFT); }
        catch (ActivityNotFoundException unavailable) {
            pendingExport = null;
            status.setText("This device has no document picker.");
            updateExport();
        }
    }

    @Override @SuppressWarnings("deprecation")
    protected void onActivityResult(int request, int result, Intent data) {
        super.onActivityResult(request, result, data);
        if (request != CREATE_DRAFT) return;
        String report = pendingExport;
        pendingExport = null;
        Uri chosen = data == null ? null : data.getData();
        if (result != RESULT_OK || chosen == null || !"content".equals(chosen.getScheme()) || report == null) {
            status.setText(result == RESULT_CANCELED ? "Draft export canceled." : "No reviewed export was confirmed.");
            updateExport();
            return;
        }
        writingExport = true;
        updateExport();
        status.setText("Saving the reviewed draft…");
        DISK.execute(() -> {
            boolean saved;
            try (OutputStream out = getContentResolver().openOutputStream(chosen, "wt")) {
                if (out == null) throw new IOException("No output stream");
                out.write(report.getBytes(StandardCharsets.UTF_8));
                saved = true;
            } catch (IOException | RuntimeException failure) { saved = false; }
            boolean confirmed = saved;
            runOnUiThread(() -> {
                if (isDestroyed()) return;
                writingExport = false;
                status.setText(confirmed ? "Reviewed draft saved." : "Draft export failed. Check the chosen destination.");
                updateExport();
            });
        });
    }

    private void updateExport() {
        export.setEnabled(draft != null && !draft.steps.isEmpty() && !pending && pendingExport == null && !writingExport);
    }

    private void render(String message) {
        status.setText(message);
        updateExport();
        rows.removeAllViews();
        add.setEnabled(!pending && draft.steps.size() < RepairDraft.LIMIT);
        for (int i = 0; i < draft.steps.size(); ++i) {
            int slot = i;
            RepairDraft.Step step = draft.steps.get(i);
            text(rows, (i + 1) + ". " + step.kind.label + " · " + step.label, 17, WHITE, true);
            var controls = new LinearLayout(this);
            controls.setOrientation(LinearLayout.HORIZONTAL);
            rows.addView(controls);
            Button up = button(controls, "Up", () -> save(draft.move(slot, -1), false));
            up.setContentDescription("Move step " + (i + 1) + " up");
            up.setEnabled(!pending && i > 0);
            Button down = button(controls, "Down", () -> save(draft.move(slot, 1), false));
            down.setContentDescription("Move step " + (i + 1) + " down");
            down.setEnabled(!pending && i + 1 < draft.steps.size());
            Button remove = button(controls, "Remove", () -> new AlertDialog.Builder(this)
                .setTitle("Remove this step?")
                .setNegativeButton("Keep", null)
                .setPositiveButton("Remove", (dialog, which) -> save(draft.remove(slot), false)).show());
            remove.setContentDescription("Remove step " + (i + 1));
            remove.setEnabled(!pending);
        }
    }

    private int dp(int value) { return Math.round(getResources().getDisplayMetrics().density * value); }
    private LinearLayout column() { LinearLayout l = new LinearLayout(this); l.setOrientation(1); return l; }
    private TextView text(LinearLayout parent, String value, int size, int color, boolean bold) {
        TextView t = new TextView(this);
        t.setText(value); t.setTextSize(size); t.setTextColor(color);
        t.setPadding(0, dp(5), 0, dp(5));
        if (bold) t.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        parent.addView(t);
        return t;
    }
    private Button button(LinearLayout parent, String value, Runnable click) {
        Button b = new Button(this);
        b.setText(value); b.setAllCaps(false); b.setMinimumHeight(dp(48));
        b.setOnClickListener(view -> click.run()); parent.addView(b);
        return b;
    }
}
