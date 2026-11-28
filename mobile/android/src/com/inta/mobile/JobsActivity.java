package com.inta.mobile;

import android.app.Activity;
import android.graphics.Typeface;
import android.os.Bundle;
import android.text.InputFilter;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;
import java.io.IOException;
import java.nio.file.Path;
import java.util.Map;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;

public final class JobsActivity extends Activity {
    private static final Executor DISK = Executors.newSingleThreadExecutor();
    private Path root;
    private LinearLayout jobs;
    private EditText name;
    private TextView status;
    private Button create;
    private boolean busy;

    @Override protected void onCreate(Bundle saved) {
        super.onCreate(saved);
        root = getNoBackupFilesDir().toPath();
        OpticalField field = new OpticalField(this);
        ScrollView scroll = new ScrollView(this);
        scroll.setFitsSystemWindows(true);
        field.addView(scroll);
        LinearLayout body = new LinearLayout(this);
        int pad = Math.round(20 * getResources().getDisplayMetrics().density);
        body.setPadding(pad, pad, pad, pad);
        body.setOrientation(LinearLayout.VERTICAL);
        scroll.addView(body);
        TextView title = new TextView(this);
        title.setText("Repair jobs");
        title.setTextSize(30);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        body.addView(title);
        name = new EditText(this);
        name.setSingleLine();
        name.setHint("New job name");
        name.setContentDescription("New job name");
        name.setFilters(new InputFilter[] { new InputFilter.LengthFilter(80) });
        body.addView(name);
        create = button(body, "Create job", unused -> change(null));
        status = new TextView(this);
        status.setAccessibilityLiveRegion(View.ACCESSIBILITY_LIVE_REGION_POLITE);
        body.addView(status);
        jobs = new LinearLayout(this);
        jobs.setOrientation(LinearLayout.VERTICAL);
        body.addView(jobs);
        setContentView(field);
        create.setEnabled(false);
        DISK.execute(() -> {
            try {
                JobBook book = JobBook.read(root);
                runOnUiThread(() -> {
                    if (isDestroyed()) return;
                    for (Map.Entry<String, String> entry : book.jobs.entrySet())
                        button(jobs, entry.getValue(), unused -> change(entry.getKey()));
                    status.setText("Current job · " + book.jobs.get(book.current));
                    create.setEnabled(true);
                });
            } catch (IOException e) { showError("Could not open the saved job index."); }
        });
    }

    private Button button(LinearLayout parent, String text, View.OnClickListener action) {
        Button result = new Button(this);
        result.setText(text);
        result.setAllCaps(false);
        result.setOnClickListener(action);
        parent.addView(result);
        return result;
    }

    private void change(String id) {
        if (busy) return;
        busy = true;
        String entered = name.getText().toString();
        DISK.execute(() -> {
            try {
                if (id == null) JobBook.create(root, entered); else JobBook.choose(root, id);
                runOnUiThread(() -> { if (!isDestroyed()) { setResult(RESULT_OK); finish(); } });
            } catch (IOException problem) { showError(problem.getMessage()); }
        });
    }
    private void showError(String message) {
        runOnUiThread(() -> { if (!isDestroyed()) { busy = false; status.setText(message); } });
    }
}
