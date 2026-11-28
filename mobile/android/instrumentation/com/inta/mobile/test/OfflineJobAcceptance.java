package com.inta.mobile.test;

import android.app.Activity;
import android.app.Instrumentation;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.os.Bundle;
import android.os.SystemClock;
import android.view.View;
import android.view.ViewGroup;
import android.widget.CheckBox;
import android.widget.TextView;
import java.util.concurrent.atomic.AtomicReference;

/** Instrument an isolated Android guest, never install into an owner's app profile. */
public final class OfflineJobAcceptance extends Instrumentation {
    private String stage;

    @Override public void onCreate(Bundle args) {
        super.onCreate(args);
        stage = args == null ? "" : args.getString("stage", "");
        start();
    }

    @Override public void onStart() {
        Bundle result = new Bundle();
        try {
            Activity activity = startActivitySync(new Intent()
                .setClassName(getTargetContext(), "com.inta.mobile.JobActivity")
                .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));
            require(getTargetContext().checkSelfPermission("android.permission.INTERNET")
                == PackageManager.PERMISSION_DENIED, "unexpected network access");
            if ("start".equals(stage)) {
                find(activity, "Paused on this phone.", false);
                press(activity, "Start timer");
                find(activity, "Pause timer", true);
                press(activity, "Confirm scope and back up");
                find(activity, "1 of 6 complete", false);
                require(checked(activity), "visible checklist selection not persisted");
            } else if ("resume".equals(stage)) {
                find(activity, "Pause timer", true);
                require(checked(activity), "cold start lost the checklist");
                require(seconds(activity) > 0, "cold start lost the running interval");
                press(activity, "Pause timer");
                find(activity, "Start timer", true);
                final long paused = seconds(activity);
                SystemClock.sleep(1150);
                require(seconds(activity) == paused, "paused timer changed");
                for (String step : new String[]{"Scan and remove threats", "Diagnose and repair the reported issue",
                        "Apply verified updates and drivers", "Retest, reboot if needed, and clean up",
                        "Review billable time and prepare invoice"}) {
                    press(activity, step);
                }
                find(activity, "6 of 6 complete", false);
                press(activity, "Mark job complete");
                find(activity, "Completed. Review recorded time before billing.", false);
                require(seconds(activity) == paused, "completion changed paused time");
            } else if ("verify-completed".equals(stage)) {
                find(activity, "Completed. Review recorded time before billing.", false);
                find(activity, "6 of 6 complete", false);
                require(!enabled(activity, "Mark job complete"), "completed job remained mutable");
                require(seconds(activity) > 0, "completed record lost time");
            } else {
                throw new IllegalArgumentException("unknown acceptance stage");
            }
            result.putString("stream", "\nINTA_ANDROID_ACCEPTANCE_OK " + stage + "\n");
            finish(Activity.RESULT_OK, result);
        } catch (Throwable failure) {
            result.putString("stream", "\nINTA_ANDROID_ACCEPTANCE_FAILED " + stage + " " + failure + "\n");
            finish(Activity.RESULT_CANCELED, result);
        }
    }

    private void press(Activity activity, String label) {
        TextView view = find(activity, label, true);
        runOnMainSync(() -> {
            boolean handled = view.performClick();
            // A checkbox toggles before dispatching an optional click listener.
            // The queue assertions verify the durable selection instead.
            require(view instanceof CheckBox || handled, "click not handled: " + label);
        });
    }

    private boolean checked(Activity activity) {
        TextView view = find(activity, "Confirm scope and back up", false);
        AtomicReference<Boolean> result = new AtomicReference<>(false);
        runOnMainSync(() -> result.set(view instanceof CheckBox && ((CheckBox) view).isChecked()));
        return result.get();
    }

    private boolean enabled(Activity activity, String label) {
        TextView view = find(activity, label, false);
        AtomicReference<Boolean> result = new AtomicReference<>(false);
        runOnMainSync(() -> result.set(view.isEnabled()));
        return result.get();
    }

    private TextView find(Activity activity, String text, boolean mustBeEnabled) {
        long deadline = SystemClock.uptimeMillis() + 10_000;
        do {
            AtomicReference<TextView> result = new AtomicReference<>();
            runOnMainSync(() -> {
                TextView view = search(activity.getWindow().getDecorView(), text, false);
                if (view != null && (!mustBeEnabled || view.isEnabled())) result.set(view);
            });
            if (result.get() != null) return result.get();
            SystemClock.sleep(100);
        } while (SystemClock.uptimeMillis() < deadline);
        throw new AssertionError("missing native text: " + text);
    }

    private long seconds(Activity activity) {
        AtomicReference<String> clock = new AtomicReference<>();
        runOnMainSync(() -> {
            TextView found = search(activity.getWindow().getDecorView(), "", true);
            if (found != null) clock.set(found.getText().toString());
        });
        String text = clock.get();
        require(text != null, "missing clock text");
        String[] fields = text.split(":");
        return Long.parseLong(fields[0]) * 3600 + Long.parseLong(fields[1]) * 60 + Long.parseLong(fields[2]);
    }

    private static TextView search(View view, String text, boolean clock) {
        if (view instanceof TextView) {
            TextView label = (TextView) view;
            String value = label.getText().toString();
            if (clock ? value.matches("[0-9]{2,}:[0-9]{2}:[0-9]{2}") : text.equals(value)) return label;
        }
        if (view instanceof ViewGroup) {
            ViewGroup group = (ViewGroup) view;
            for (int index = 0; index < group.getChildCount(); index++) {
                TextView found = search(group.getChildAt(index), text, clock);
                if (found != null) return found;
            }
        }
        return null;
    }

    private static void require(boolean condition, String message) {
        if (!condition) throw new AssertionError(message);
    }
}
