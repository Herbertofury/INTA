package com.inta.mobile.test;

import android.app.Activity;
import android.app.Instrumentation;
import android.accessibilityservice.AccessibilityServiceInfo;
import android.content.ContentValues;
import android.content.Intent;
import android.content.IntentFilter;
import android.content.pm.PackageManager;
import android.graphics.Rect;
import android.net.Uri;
import android.os.Bundle;
import android.os.SystemClock;
import android.provider.MediaStore;
import android.view.View;
import android.view.MotionEvent;
import android.view.ViewGroup;
import android.view.accessibility.AccessibilityNodeInfo;
import android.widget.CheckBox;
import android.widget.EditText;
import android.widget.Spinner;
import android.widget.TextView;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
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
                .setClassName(getTargetContext(), "diagnostic-import".equals(stage)
                    ? "com.inta.mobile.DiagnosticHandoffActivity" : stage.startsWith("draft")
                    ? "com.inta.mobile.RepairDraftActivity" : "com.inta.mobile.JobActivity")
                .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));
            require(getTargetContext().checkSelfPermission("android.permission.INTERNET")
                == PackageManager.PERMISSION_DENIED, "unexpected network access");
            if ("diagnostic-import".equals(stage)) {
                verifyDiagnosticImport(activity);
            } else if (stage.startsWith("jobs-")) {
                verifyJobSwitch(activity);
            } else if ("optics".equals(stage)) {
                var field = getTargetContext().getClassLoader().loadClass("com.inta.mobile.OpticalField");
                var nativeLoaded = field.getDeclaredField("NATIVE");
                nativeLoaded.setAccessible(true);
                require(nativeLoaded.getBoolean(null), "Android failed to load the packaged Rust field");
                var renderer = field.getDeclaredMethod("cell", int.class, int.class, int.class,
                        int.class, int.class, long.class, long.class);
                renderer.setAccessible(true);
                int active = 0;
                long event = 32768L | (32768L << 16) | (250L << 32) | (255L << 48);
                for (int y = 0; y < 12; y++) for (int x = 0; x < 24; x++) {
                    int idle = (Integer)renderer.invoke(null, x, y, 24, 12, 0, 0L, 0L);
                    int moving = (Integer)renderer.invoke(null, x, y, 24, 12, 0, event, 0L);
                    if (idle != moving) active++;
                }
                require(active > 8, "native interaction did not change the Android glyph field");
                press(activity, "Appearance");
                tapOption("Classic");
                awaitProfile(activity, 4);
                press(activity, "Appearance");
                tapOption("Full Prism");
                awaitProfile(activity, 0);
            } else if ("draft".equals(stage)) {
                find(activity, "Saved on this phone.", false);
                edit(activity, 3, "Scan the serviced PC");
                press(activity, "Add to draft");
                find(activity, "1. Run a malware scan · Scan the serviced PC", false);
                edit(activity, 1, "Review printer queue");
                press(activity, "Add to draft");
                find(activity, "2. Apply a repair · Review printer queue", false);
                find(activity, "Add to draft", true);
                runOnMainSync(() -> {
                    View move = byDescription(activity.getWindow().getDecorView(), "Move step 2 up");
                    require(move != null && move.isEnabled() && move.performClick(), "cannot move second repair step");
                });
                find(activity, "1. Apply a repair · Review printer queue", false);
            } else if ("draft-verify".equals(stage)) {
                find(activity, "1. Apply a repair · Review printer queue", false);
                find(activity, "2. Run a malware scan · Scan the serviced PC", false);
                find(activity, "Put steps in order before picking and approving a PC. Nothing runs from this draft.", false);
            } else if ("invoice".equals(stage)) {
                find(activity, "Completed. Review recorded time before billing.", false);
                find(activity, "Save invoice draft", true);
                long frozen=seconds(activity);
                runOnMainSync(() -> {
                    View root=activity.getWindow().getDecorView();
                    EditText rate=(EditText)byDescription(root, "Hourly rate (USD)");
                    EditText minutes=(EditText)byDescription(root, "Billable minutes (optional)");
                    require(rate != null && minutes != null && rate.isShown(), "billing form missing");
                    rate.setText("95.00");
                    minutes.setText("5");
                });
                press(activity, "Business billing (unchecked: residential)");
                find(activity, "Unsaved preview · Draft subtotal US$7.92", false);
                press(activity, "Save invoice draft");
                find(activity, "Saved on this phone · Draft subtotal US$7.92 · 5 billable min", false);
                require(seconds(activity)==frozen, "billing cannot rewrite recorded time");
            } else if ("invoice-verify".equals(stage)) {
                find(activity, "Saved on this phone · Draft subtotal US$7.92 · 5 billable min", false);
                runOnMainSync(() -> {
                    View root=activity.getWindow().getDecorView();
                    EditText rate=(EditText)byDescription(root, "Hourly rate (USD)");
                    EditText minutes=(EditText)byDescription(root, "Billable minutes (optional)");
                    CheckBox profile=(CheckBox)search(root, "Business billing (unchecked: residential)", false);
                    require(rate != null && "95.00".contentEquals(rate.getText()), "rate did not survive cold launch");
                    require(minutes != null && "5".contentEquals(minutes.getText()), "manual minutes did not survive");
                    require(profile != null && profile.isChecked(), "business category did not survive");
                });
            } else if ("handoff".equals(stage)) {
                verifyHandoff(activity);
            } else if ("start".equals(stage)) {
                find(activity, "Paused on this phone.", false);
                TextView billing=find(activity,"Invoice draft",false);
                runOnMainSync(() -> require(!billing.isShown(),"unfinished jobs must not show billing"));
                require(!enabled(activity, "Review handoff export"), "unfinished job export was enabled");
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

    private void verifyHandoff(Activity activity) throws Exception {
        long recorded = seconds(activity);
        var resolver = getTargetContext().getContentResolver();
        ContentValues file = new ContentValues();
        file.put(MediaStore.Downloads.DISPLAY_NAME, "inta-ci-handoff-" + SystemClock.elapsedRealtime() + ".txt");
        file.put(MediaStore.Downloads.MIME_TYPE, "text/plain");
        file.put(MediaStore.Downloads.IS_PENDING, 1);
        Uri uri = resolver.insert(MediaStore.Downloads.EXTERNAL_CONTENT_URI, file);
        require(uri != null, "could not allocate an isolated export file");
        try {
            byte[] original = "Untouched acceptance-test destination".getBytes(StandardCharsets.UTF_8);
            try (OutputStream bytes = resolver.openOutputStream(uri, "wt")) {
                require(bytes != null, "document provider could not create the test file");
                bytes.write(original);
            }
            press(activity, "Review handoff export");
            clickDialog("Cancel", "android:id/button2");
            find(activity, "Review handoff export", true);
            try (InputStream bytes = resolver.openInputStream(uri)) {
                require(bytes != null && Arrays.equals(bytes.readAllBytes(), original), "cancel changed the test file");
            }
            IntentFilter create = new IntentFilter(Intent.ACTION_CREATE_DOCUMENT, "text/plain");
            create.addCategory(Intent.CATEGORY_OPENABLE);
            var monitor = addMonitor(create,
                    new ActivityResult(Activity.RESULT_OK, new Intent().setData(uri)), true);
            try {
                press(activity, "Review handoff export");
                clickDialog("Choose save location", "android:id/button1");
                find(activity, "Reviewed handoff saved.", false);
                require(monitor.getHits() == 1, "Android create-document action was not launched");
            } finally { removeMonitor(monitor); }
            try (InputStream bytes = resolver.openInputStream(uri)) {
                require(bytes != null, "document provider did not return an export");
                String report = new String(bytes.readAllBytes(), StandardCharsets.UTF_8);
                require(report.startsWith("INTA service handoff\n") && report.contains("[x] Confirm scope and back up")
                    && report.contains("Billing profile: Business") && report.contains("Billable minutes: 5")
                    && report.contains("Draft subtotal: 7.92"), "unexpected handoff contents");
            }
            require(seconds(activity) == recorded, "export altered the source timer");
        } finally {
            resolver.delete(uri, null, null);
        }
    }

    private void awaitProfile(Activity activity, int expected) {
        var prefs = getTargetContext().getSharedPreferences("optics-v1", 0);
        long deadline = SystemClock.uptimeMillis() + 5000;
        do {
            AtomicReference<Boolean> ready = new AtomicReference<>(false);
            runOnMainSync(() -> ready.set(activity.hasWindowFocus() && prefs.getInt("profile", -1) == expected));
            if (ready.get()) return;
            SystemClock.sleep(60);
        } while (SystemClock.uptimeMillis() < deadline);
        throw new AssertionError("appearance did not settle at " + expected + ", stored " + prefs.getInt("profile", -1));
    }

    private void tapOption(String name) {
        long deadline = SystemClock.uptimeMillis() + 8000;
        do {
            AccessibilityNodeInfo root = getUiAutomation().getRootInActiveWindow();
            if (root != null) {
                for (AccessibilityNodeInfo item : root.findAccessibilityNodeInfosByText(name)) {
                    if (!name.contentEquals(item.getText() == null ? "" : item.getText()) || !item.isVisibleToUser()) continue;
                    Rect bounds = new Rect();
                    item.getBoundsInScreen(bounds);
                    if (bounds.isEmpty()) continue;
                    long now = SystemClock.uptimeMillis();
                    for (int action : new int[]{MotionEvent.ACTION_DOWN, MotionEvent.ACTION_UP}) {
                        MotionEvent touch = MotionEvent.obtain(now, SystemClock.uptimeMillis(), action,
                                bounds.centerX(), bounds.centerY(), 0);
                        try { sendPointerSync(touch); } finally { touch.recycle(); }
                    }
                    return;
                }
            }
            SystemClock.sleep(100);
        } while (SystemClock.uptimeMillis() < deadline);
        throw new AssertionError("appearance option was not visible: " + name);
    }

    private void verifyDiagnosticImport(Activity activity) throws Exception {
        press(activity,"Create one-time pairing");
        clickDialog("Create pairing","android:id/button1");
        find(activity,"Choose encrypted report",true);
        var code=new AtomicReference<String>();
        runOnMainSync(() -> code.set(((TextView)activity.getWindow().getDecorView().findViewWithTag("pairing-code")).getText().toString()));
        String[] parts=code.get().split("\\.");
        require(parts.length==3 && "INTA1".equals(parts[0]) && parts[2].matches("[0-9a-f]{32}"),"Missing ephemeral pairing");
        long issued=Long.parseLong(parts[1]);byte[] key=new byte[16];
        for(int n=0;n<key.length;n++) key[n]=(byte)Integer.parseInt(parts[2].substring(n*2,n*2+2),16);
        var hash=java.security.MessageDigest.getInstance("SHA-256");hash.update("INTA-DIAGNOSTIC-SESSION-1".getBytes(StandardCharsets.US_ASCII));
        byte[] nonce=new byte[12];new java.security.SecureRandom().nextBytes(nonce);
        byte[] header=java.nio.ByteBuffer.allocate(44).put("INTADG01".getBytes(StandardCharsets.US_ASCII)).putLong(issued)
                .put(hash.digest(key),0,16).put(nonce).array();
        var cipher=javax.crypto.Cipher.getInstance("AES/GCM/NoPadding");
        cipher.init(javax.crypto.Cipher.ENCRYPT_MODE,new javax.crypto.spec.SecretKeySpec(key,"AES"),new javax.crypto.spec.GCMParameterSpec(128,nonce));
        Arrays.fill(key,(byte)0);cipher.updateAAD(header);
        byte[] plain=java.nio.ByteBuffer.allocate(27).putLong(issued).putLong(8L<<30).putLong(4L<<30).put((byte)50).put((byte)1).put((byte)100).array();
        byte[] sealed=java.nio.ByteBuffer.allocate(87).put(header).put(cipher.doFinal(plain)).array();
        var resolver=getTargetContext().getContentResolver();
        var values=new ContentValues();
        values.put(MediaStore.Downloads.DISPLAY_NAME,"inta-diagnostic-test-"+SystemClock.elapsedRealtime()+".intadiag");
        values.put(MediaStore.Downloads.MIME_TYPE,"application/octet-stream");
        values.put(MediaStore.Downloads.IS_PENDING,1);
        Uri uri=resolver.insert(MediaStore.Downloads.EXTERNAL_CONTENT_URI,values);
        require(uri!=null,"Could not create the scoped fixture");
        try {
            try(var out=resolver.openOutputStream(uri,"wt")){require(out!=null,"Could not write fixture");out.write(sealed);}
            IntentFilter select=new IntentFilter(Intent.ACTION_OPEN_DOCUMENT,"*/*");select.addCategory(Intent.CATEGORY_OPENABLE);
            var monitor=addMonitor(select,new ActivityResult(Activity.RESULT_OK,new Intent().setData(uri)),true);
            try {
                press(activity,"Choose encrypted report");find(activity,"Report authenticated. The pairing has been consumed.",false);
                require(monitor.getHits()==1 && !enabled(activity,"Choose encrypted report"),"SAF or one-use invariant failed");
            } finally {removeMonitor(monitor);}
            press(activity,"Create one-time pairing");clickDialog("Create pairing","android:id/button1");
            press(activity,"Revoke pairing");require(!enabled(activity,"Choose encrypted report"),"Revoked pairing still active");
        } finally {require(resolver.delete(uri,null,null)==1,"Could not release fixture");}
    }

    private void clickDialog(String name, String buttonId) {
        var automation = getUiAutomation();
        var service = automation.getServiceInfo();
        service.flags |= AccessibilityServiceInfo.FLAG_REPORT_VIEW_IDS;
        automation.setServiceInfo(service);
        long deadline = SystemClock.uptimeMillis() + 8000;
        do {
            AccessibilityNodeInfo root = automation.getRootInActiveWindow();
            if (root != null) {
                for (AccessibilityNodeInfo node : root.findAccessibilityNodeInfosByViewId(buttonId)) {
                    CharSequence text = node.getText();
                    if (text != null && name.equalsIgnoreCase(text.toString())
                            && node.performAction(AccessibilityNodeInfo.ACTION_CLICK)) return;
                }
            }
            SystemClock.sleep(100);
        } while (SystemClock.uptimeMillis() < deadline);
        throw new AssertionError("missing confirmation button: " + name);
    }

    private void edit(Activity activity, int type, String name) {
        find(activity, "Add to draft", true);
        runOnMainSync(() -> {
            View root = activity.getWindow().getDecorView();
            Spinner picker = byType(root, Spinner.class);
            EditText input = byType(root, EditText.class);
            require(picker != null && input != null, "draft editor controls missing");
            picker.setSelection(type);
            input.setText(name);
        });
    }

    private static View byDescription(View view, String value) {
        if (value.contentEquals(view.getContentDescription() == null ? "" : view.getContentDescription())) return view;
        if (view instanceof ViewGroup) {
            ViewGroup group = (ViewGroup) view;
            for (int i = 0; i < group.getChildCount(); ++i) {
                View found = byDescription(group.getChildAt(i), value);
                if (found != null) return found;
            }
        }
        return null;
    }

    private static <T extends View> T byType(View view, Class<T> kind) {
        if (kind.isInstance(view)) return kind.cast(view);
        if (view instanceof ViewGroup) {
            ViewGroup group = (ViewGroup) view;
            for (int i = 0; i < group.getChildCount(); ++i) {
                T found = byType(group.getChildAt(i), kind);
                if (found != null) return found;
            }
        }
        return null;
    }

    private void verifyJobSwitch(Activity activity) {
        if ("jobs-return".equals(stage)) {
            find(activity, "6 of 6 complete", false);
            find(activity, "Saved on this phone · Draft subtotal US$7.92 · 5 billable min", false);
            return;
        }
        boolean create = "jobs-create".equals(stage);
        if (create) {
            find(activity, "Saved on this phone · Draft subtotal US$7.92 · 5 billable min", false);
        } else {
            require("jobs-switch".equals(stage), "unknown work order test");
            find(activity, "Laptop 812", false);
            find(activity, "0 of 6 complete", false);
            press(activity, "Confirm scope and back up");
            find(activity, "1 of 6 complete", false);
        }
        ActivityMonitor monitor = addMonitor("com.inta.mobile.JobsActivity", null, false);
        Activity list;
        try {
            press(activity, "Switch or create job");
            list = waitForMonitorWithTimeout(monitor, 10000);
            require(list != null, "job selector did not open");
        } finally { removeMonitor(monitor); }
        if (create) {
            find(list, "Create job", true);
            runOnMainSync(() -> ((EditText) byDescription(list.getWindow().getDecorView(), "New job name")).setText("Laptop 812"));
            press(list, "Create job");
        } else press(list, "Repair job");
        long deadline = SystemClock.elapsedRealtime() + 5000;
        while (!list.isFinishing() && !list.isDestroyed() && SystemClock.elapsedRealtime() < deadline) SystemClock.sleep(50);
        require(list.isFinishing() || list.isDestroyed(), "job selection did not finish");
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
