package com.inta.mobile;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.Intent;
import android.content.res.ColorStateList;
import android.graphics.Color;
import android.graphics.Typeface;
import android.graphics.drawable.GradientDrawable;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.os.SystemClock;
import android.text.Editable;
import android.text.InputFilter;
import android.text.InputType;
import android.text.TextWatcher;
import android.view.View;
import android.provider.Settings;
import android.view.WindowInsets;
import android.widget.Button;
import android.widget.CheckBox;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.Locale;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;

public final class JobActivity extends Activity {
    private static final int WHITE = 0xfff5f7fa, MUTED = 0xffa8b1c1;
    private static final int CARD = 0xf3151b25, ACCENT = 0xff9af4d8;
    private static final String[] STEPS = {
        "Confirm scope and back up",
        "Scan and remove threats",
        "Diagnose and repair the reported issue",
        "Apply verified updates and drivers",
        "Retest, reboot if needed, and clean up",
        "Review billable time and prepare invoice"
    };
    private static final Executor DISK = Executors.newSingleThreadExecutor();
    private static final int CREATE_HANDOFF = 31;
    private final Handler ui = new Handler(Looper.getMainLooper());
    private final CheckBox[] checks = new CheckBox[STEPS.length];
    private TextView clock, detail, progress;
    private Button timer, complete;
    private LinearLayout billing;
    private EditText rate, billableMinutes;
    private CheckBox business;
    private TextView billingStatus;
    private Button saveInvoice;
    private boolean billingLoaded, billingPending, billingDamaged;
    private Button handoff;
    private TextView handoffStatus;
    private InvoiceDraft savedInvoice;
    private String pendingHandoff;
    private boolean writingHandoff;
    private JobStore store;
    private JobTimer state;
    private int boot;
    private boolean visible, updating, pending, failed;
    private final Runnable tick = new Runnable() {
        @Override public void run() {
            if (!visible || state == null || state.phase != JobTimer.Phase.RUNNING) return;
            showClock();
            ui.postDelayed(this, 1000);
        }
    };

    @Override protected void onCreate(Bundle saved) {
        super.onCreate(saved);
        if (saved != null) pendingHandoff = saved.getString("pending-handoff");
        ScrollView scroll = new ScrollView(this);
        OpticalField optics = new OpticalField(this);
        optics.addView(scroll);
        scroll.setFillViewport(true);
        scroll.setOnApplyWindowInsetsListener((view, insets) -> {
            if (Build.VERSION.SDK_INT >= 30) {
                var area = insets.getInsets(WindowInsets.Type.systemBars() | WindowInsets.Type.displayCutout());
                view.setPadding(area.left, area.top, area.right, area.bottom);
            } else {
                applyLegacyInsets(view, insets);
            }
            return insets;
        });
        LinearLayout body = column();
        body.setPadding(dp(20), dp(19), dp(20), dp(24));
        scroll.addView(body);
        label(body, "INTA MOBILE  /  OFFLINE PREVIEW", 12, ACCENT, true);
        label(body, "Repair job", 33, WHITE, true);
        label(body, "Track work here while the serviced PC restarts.", 15, MUTED, false);
        optics.addAppearanceButton(body);
        LinearLayout timerCard = card(body);
        label(timerCard, "RECORDED TIME", 12, MUTED, true);
        clock = label(timerCard, "00:00:00", 38, WHITE, true);
        clock.setTypeface(Typeface.MONOSPACE, Typeface.BOLD);
        detail = label(timerCard, "Opening local job…", 14, MUTED, false);
        timer = button(timerCard, "Start timer", () -> {
            long now = SystemClock.elapsedRealtime();
            save(state.phase == JobTimer.Phase.RUNNING ? state.pause(now, boot) : state.start(now, boot));
        });
        LinearLayout checklist = card(body);
        label(checklist, "Technician checklist", 21, WHITE, true);
        progress = label(checklist, "0 of 6 complete", 14, MUTED, false);
        for (int index = 0; index < STEPS.length; index++) {
            final int step = index;
            CheckBox box = new CheckBox(this);
            box.setText(STEPS[index]);
            box.setTextColor(WHITE);
            box.setButtonTintList(ColorStateList.valueOf(ACCENT));
            box.setTextSize(15);
            box.setMinimumHeight(dp(48));
            box.setOnCheckedChangeListener((view, checked) -> {
                if (!updating && !pending && state != null) save(state.check(step, checked));
            });
            checks[index] = box;
            checklist.addView(box, new LinearLayout.LayoutParams(-1, -2));
        }
        complete = button(checklist, "Mark job complete", () -> {
            if (state.steps == JobTimer.ALL_STEPS) finishJob();
            else new AlertDialog.Builder(this).setTitle("Finish with unchecked steps?")
                .setMessage("The timer will stop. Unchecked steps remain recorded as incomplete.")
                .setNegativeButton("Keep working", null).setPositiveButton("Finish job", (dialog, which) -> finishJob()).show();
        });
        billing = card(body);
        label(billing, "Invoice draft", 22, WHITE, true);
        label(billing, "Uses recorded time unless you override it. Nothing is sent or charged.", 14, MUTED, false);
        rate = billingField(billing, "Hourly rate (USD)", InputType.TYPE_CLASS_NUMBER | InputType.TYPE_NUMBER_FLAG_DECIMAL);
        billableMinutes = billingField(billing, "Billable minutes (optional)", InputType.TYPE_CLASS_NUMBER);
        business = new CheckBox(this);
        business.setText("Business billing (unchecked: residential)");
        business.setTextColor(WHITE);
        business.setMinimumHeight(dp(48));
        billing.addView(business);
        saveInvoice = button(billing, "Save invoice draft", this::saveBilling);
        billingStatus = label(billing, "Enter your rate to calculate the subtotal.", 14, ACCENT, true);
        TextWatcher preview = new TextWatcher() {
            @Override public void beforeTextChanged(CharSequence text, int start, int count, int after) { }
            @Override public void onTextChanged(CharSequence text, int start, int before, int count) { refreshBilling(); }
            @Override public void afterTextChanged(Editable text) { }
        };
        rate.addTextChangedListener(preview);
        billableMinutes.addTextChangedListener(preview);
        business.setOnCheckedChangeListener((button, enabled) -> refreshBilling());
        LinearLayout handoffCard = card(body);
        label(handoffCard, "Service handoff", 22, WHITE, true);
        handoff = button(handoffCard, "Review handoff export", this::reviewHandoff);
        handoffStatus = label(handoffCard, "Complete the job to export its recorded time and checklist.", 14, MUTED, false);
        button(body, "Draft a repair plan", () -> startActivity(new Intent(this, RepairDraftActivity.class)));
        label(body, "Job records stay on this phone. An export goes to the location you choose.", 13, MUTED, false);
        setContentView(optics);
        render();
        DISK.execute(() -> {
            JobStore local = new JobStore(getNoBackupFilesDir().toPath().resolve("offline-job.v2"));
            JobTimer loaded;
            int phoneBoot;
            try {
                phoneBoot = Settings.Global.getInt(getContentResolver(), Settings.Global.BOOT_COUNT, -1);
                loaded = local.read();
            } catch (IllegalArgumentException | IOException malformed) {
                ui.post(() -> { if (!isDestroyed()) { failed = true; render(); } });
                return;
            }
            int currentBoot = phoneBoot;
            JobTimer before = loaded;
            JobTimer reconciled = loaded.reconcile(SystemClock.elapsedRealtime(), phoneBoot);
            boolean ok = reconciled == before || write(local, reconciled);
            ui.post(() -> {
                if (isDestroyed()) return;
                store = local;
                boot = currentBoot;
                state = ok ? reconciled : null;
                failed = !ok;
                render();
            });
        });
    }

    @Override protected void onStart() { super.onStart(); visible = true; render(); }

    @Override protected void onSaveInstanceState(Bundle saved) {
        super.onSaveInstanceState(saved);
        saved.putString("pending-handoff", pendingHandoff);
    }

    @SuppressWarnings("deprecation")
    private void reviewHandoff() {
        if (state == null || !billingLoaded || billingPending || writingHandoff || pendingHandoff != null) return;
        String report = JobHandoff.render(state, savedInvoice, STEPS);
        new AlertDialog.Builder(this).setTitle("Review service handoff").setMessage(report)
                .setNegativeButton("Cancel", null).setPositiveButton("Choose save location", (dialog, which) -> {
                    pendingHandoff = report;
                    updateHandoff();
                    Intent create = new Intent(Intent.ACTION_CREATE_DOCUMENT).addCategory(Intent.CATEGORY_OPENABLE)
                            .setType("text/plain").putExtra(Intent.EXTRA_TITLE, "INTA-service-handoff.txt");
                    try {
                        startActivityForResult(create, CREATE_HANDOFF);
                    } catch (android.content.ActivityNotFoundException unavailable) {
                        pendingHandoff = null;
                        handoffStatus.setText("This device has no document picker.");
                        updateHandoff();
                    }
                }).show();
    }

    @Override @SuppressWarnings("deprecation")
    protected void onActivityResult(int request, int result, Intent data) {
        super.onActivityResult(request, result, data);
        if (request != CREATE_HANDOFF) return;
        String report = pendingHandoff;
        pendingHandoff = null;
        Uri destination = data == null ? null : data.getData();
        if (result != RESULT_OK || report == null || destination == null || !"content".equals(destination.getScheme())) {
            handoffStatus.setText(result == RESULT_CANCELED ? "Export canceled." : "No document destination was confirmed.");
            updateHandoff();
            return;
        }
        writingHandoff = true;
        updateHandoff();
        handoffStatus.setText("Saving the reviewed handoff…");
        DISK.execute(() -> {
            boolean saved = false;
            try (OutputStream stream = getContentResolver().openOutputStream(destination, "wt")) {
                if (stream == null) throw new IOException("Document provider returned no stream");
                stream.write(report.getBytes(StandardCharsets.UTF_8));
                stream.flush();
                saved = true;
            } catch (IOException | RuntimeException failure) {
                saved = false;
            }
            boolean confirmed = saved;
            ui.post(() -> {
                if (isDestroyed()) return;
                writingHandoff = false;
                handoffStatus.setText(confirmed ? "Reviewed handoff saved." : "Export failed. Check the chosen destination before retrying.");
                updateHandoff();
            });
        });
    }

    private void updateHandoff() {
        handoff.setEnabled(state != null && state.phase == JobTimer.Phase.COMPLETE && billingLoaded
                && !billingPending && !writingHandoff && pendingHandoff == null);
    }
    @Override protected void onStop() {
        visible = false;
        ui.removeCallbacks(tick);
        long now = SystemClock.elapsedRealtime();
        if (state != null && !pending && state.phase == JobTimer.Phase.RUNNING) {
            JobTimer saved = state.checkpoint(now, boot);
            save(saved);
        }
        super.onStop();
    }

    private void finishJob() { if (!pending && state != null) save(state.complete(SystemClock.elapsedRealtime(), boot)); }
    private static boolean write(JobStore local, JobTimer value) {
        try {
            local.write(value);
            return true;
        } catch (IOException failure) {
            return false;
        }
    }

    @SuppressWarnings("deprecation")
    private static void applyLegacyInsets(android.view.View view, WindowInsets insets) {
        view.setPadding(insets.getSystemWindowInsetLeft(), insets.getSystemWindowInsetTop(),
            insets.getSystemWindowInsetRight(), insets.getSystemWindowInsetBottom());
    }

    private void save(JobTimer next) {
        if (pending || state == null || next == state) return;
        pending = true;
        render();
        DISK.execute(() -> {
            boolean ok = write(store, next);
            ui.post(() -> {
                if (isDestroyed()) return;
                if (ok) state = next;
                failed = !ok;
                pending = false;
                render();
            });
        });
    }

    private void render() {
        if (clock == null) return;
        boolean loaded = state != null;
        boolean running = loaded && state.phase == JobTimer.Phase.RUNNING;
        boolean finished = loaded && state.phase == JobTimer.Phase.COMPLETE;
        timer.setEnabled(loaded && !pending && !finished && boot >= 0);
        complete.setEnabled(loaded && !pending && !finished);
        updating = true;
        int steps = loaded ? state.steps : 0;
        for (int index = 0; index < checks.length; index++) {
            checks[index].setEnabled(loaded && !pending && !finished);
            checks[index].setChecked((steps & (1 << index)) != 0);
        }
        updating = false;
        progress.setText(Integer.bitCount(steps) + " of " + STEPS.length + " complete");
        timer.setText(running ? "Pause timer" : finished ? "Job complete" : "Start timer");
        if (state != null) showClock();
        detail.setText(failed ? "Job could not be read or saved. No time update confirmed."
            : pending ? "Saving locally…" : !loaded ? "Opening local job…"
            : finished ? "Completed. Review recorded time before billing."
            : running ? (state.interrupted ? "Running. Unverified time from a previous clock was not counted."
                : "Running, including while this screen is closed.")
            : state.interrupted ? "The clock changed. Unverified time was not counted."
            : boot < 0 ? "Cannot verify the phone clock. Checklist remains available." : "Paused on this phone.");
        if (billing != null) {
            billing.setVisibility(finished ? View.VISIBLE : View.GONE);
            if (finished && !billingLoaded && !billingPending) loadBilling(state);
        }
        updateHandoff();
        ui.removeCallbacks(tick);
        if (visible && running) ui.postDelayed(tick, 1000);
    }

    private EditText billingField(LinearLayout card, String field, int inputType) {
        label(card, field, 14, MUTED, true);
        EditText value = new EditText(this);
        value.setContentDescription(field);
        value.setSingleLine();
        value.setTextColor(WHITE);
        value.setInputType(inputType);
        value.setFilters(new InputFilter[] { new InputFilter.LengthFilter(10) });
        card.addView(value, new LinearLayout.LayoutParams(-1, -2));
        return value;
    }

    private java.nio.file.Path invoiceFile() {
        return getNoBackupFilesDir().toPath().resolve("offline-invoice-draft.v1");
    }

    private void billingEnabled(boolean enabled) {
        rate.setEnabled(enabled);
        billableMinutes.setEnabled(enabled);
        business.setEnabled(enabled);
        saveInvoice.setEnabled(enabled);
    }

    private void loadBilling(JobTimer completed) {
        billingPending = true;
        billingEnabled(false);
        DISK.execute(() -> {
            InvoiceDraft found = null;
            boolean malformed = false;
            try { found = InvoiceDraft.read(invoiceFile(), completed); }
            catch (IOException failure) { malformed = true; }
            InvoiceDraft draft = found;
            boolean damaged = malformed;
            ui.post(() -> {
                if (isDestroyed()) return;
                billingLoaded = true;
                savedInvoice = draft;
                billingDamaged = damaged;
                if (damaged) billingStatus.setText("Previous invoice draft could not be read. It was not replaced.");
                if (draft != null) {
                    rate.setText(draft.hourlyDollars());
                    if (draft.minutes >= 0) billableMinutes.setText(Long.toString(draft.minutes));
                    business.setChecked(draft.profile == InvoiceDraft.Profile.BUSINESS);
                    showSavedBilling(draft);
                }
                billingPending = false;
                billingEnabled(!damaged);
                updateHandoff();
            });
        });
    }

    private InvoiceDraft billingDraftFromFields() {
        return InvoiceDraft.prepare(state, rate.getText().toString(), billableMinutes.getText().toString(),
                business.isChecked() ? InvoiceDraft.Profile.BUSINESS : InvoiceDraft.Profile.RESIDENTIAL);
    }

    private void refreshBilling() {
        if (billingPending || billingDamaged || !billingLoaded || state == null) return;
        try {
            billingStatus.setText("Unsaved preview · Draft subtotal US$" + billingDraftFromFields().subtotalDollars());
        } catch (IllegalArgumentException invalid) {
            billingStatus.setText(invalid.getMessage());
        }
    }

    private void saveBilling() {
        if (billingPending || billingDamaged || !billingLoaded || state == null) return;
        final InvoiceDraft draft;
        try {
            draft = billingDraftFromFields();
        } catch (IllegalArgumentException invalid) {
            billingStatus.setText(invalid.getMessage());
            return;
        }
        billingPending = true;
        billingEnabled(false);
        updateHandoff();
        billingStatus.setText("Saving invoice draft locally…");
        DISK.execute(() -> {
            boolean saved = false;
            try { draft.save(invoiceFile()); saved = true; }
            catch (IOException failure) { /* Preserve UI values for another explicit save. */ }
            boolean ok = saved;
            ui.post(() -> {
                if (isDestroyed()) return;
                billingPending = false;
                billingEnabled(true);
                if (ok) {
                    savedInvoice = draft;
                    showSavedBilling(draft);
                }
                else billingStatus.setText("Could not save. No new draft confirmed.");
                updateHandoff();
            });
        });
    }

    private void showSavedBilling(InvoiceDraft draft) {
        billingStatus.setText("Saved on this phone · Draft subtotal US$" + draft.subtotalDollars()
                + (draft.minutes < 0 ? " · Recorded time" : " · " + draft.minutes + " billable min"));
    }

    private void showClock() {
        long seconds = state.elapsed(SystemClock.elapsedRealtime(), boot) / 1000;
        clock.setText(String.format(Locale.ROOT, "%02d:%02d:%02d", seconds / 3600, (seconds / 60) % 60, seconds % 60));
    }

    private int dp(int pixels) { return Math.round(pixels * getResources().getDisplayMetrics().density); }
    private LinearLayout column() {
        LinearLayout view = new LinearLayout(this);
        view.setOrientation(LinearLayout.VERTICAL);
        return view;
    }
    private LinearLayout card(LinearLayout parent) {
        LinearLayout view = column();
        GradientDrawable background = new GradientDrawable();
        background.setColor(CARD);
        background.setStroke(dp(1), 0xff344254);
        background.setCornerRadius(dp(18));
        view.setBackground(background);
        view.setPadding(dp(16), dp(12), dp(16), dp(12));
        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(-1, -2);
        params.topMargin = dp(14);
        parent.addView(view, params);
        return view;
    }
    private TextView label(LinearLayout parent, String text, int sp, int color, boolean bold) {
        TextView view = new TextView(this);
        view.setText(text);
        view.setTextSize(sp);
        view.setTextColor(color);
        view.setPadding(0, dp(4), 0, dp(4));
        if (bold) view.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        parent.addView(view, new LinearLayout.LayoutParams(-1, -2));
        return view;
    }
    private Button button(LinearLayout parent, String text, Runnable action) {
        Button view = new Button(this);
        view.setAllCaps(false);
        view.setText(text);
        view.setMinimumHeight(dp(48));
        view.setOnClickListener(unused -> action.run());
        parent.addView(view, new LinearLayout.LayoutParams(-1, -2));
        return view;
    }
}
