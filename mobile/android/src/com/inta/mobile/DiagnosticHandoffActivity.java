package com.inta.mobile;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.Intent;
import android.content.res.AssetFileDescriptor;
import android.net.Uri;
import android.os.Bundle;
import android.os.CancellationSignal;
import android.os.Handler;
import android.os.Looper;
import android.os.SystemClock;
import android.system.Os;
import android.system.OsConstants;
import android.system.StructPollfd;
import android.view.WindowManager;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;
import java.io.EOFException;
import java.io.InputStream;
import java.text.DateFormat;
import java.util.Arrays;
import java.util.Date;
import java.util.Locale;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

/** The user picks a local ciphertext through SAF. No report, key or URI is persisted. */
public final class DiagnosticHandoffActivity extends Activity {
    private static final int PICK = 831;
    private final ExecutorService disk = Executors.newSingleThreadExecutor();
    private final Handler ui = new Handler(Looper.getMainLooper());
    private DiagnosticHandoff pairing;
    private TextView code, status, results;
    private Button create, open, revoke;
    private CancellationSignal pending;
    private final Runnable expire = () -> {
        if (pairing != null) {
            pairing.revoke(); pairing = null;
            if (pending != null) pending.cancel();
            status.setText("Pairing expired. Create another pairing to receive a new report.");
            refresh();
        }
    };
    private static long now() { return System.currentTimeMillis() / 1000L; }
    private int dp(int n) { return Math.round(getResources().getDisplayMetrics().density * n); }
    private TextView label(LinearLayout panel, String text, int size) {
        TextView v = new TextView(this); v.setTextSize(size); v.setTextColor(0xfff5f7fa); v.setText(text); v.setPadding(0,dp(4),0,dp(6)); panel.addView(v); return v;
    }
    private Button button(LinearLayout panel, String title, Runnable action) {
        Button v = new Button(this); v.setText(title); v.setAllCaps(false); v.setOnClickListener(view -> action.run()); panel.addView(v); return v;
    }
    @Override protected void onCreate(Bundle state) {
        super.onCreate(state);
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_SECURE);
        ScrollView scroll = new ScrollView(this); scroll.setFitsSystemWindows(true);
        OpticalField field = new OpticalField(this); field.addView(scroll);
        LinearLayout panel = new LinearLayout(this); panel.setOrientation(LinearLayout.VERTICAL); panel.setPadding(dp(18),dp(26),dp(18),dp(22)); scroll.addView(panel);
        label(panel, "PC diagnostics handoff", 26);
        label(panel, "Create a one-time pairing and enter its code on the Windows PC. Transfer the encrypted .intadiag file separately, then choose it here. The pairing expires in ten minutes.", 16);
        create = button(panel, "Create one-time pairing", this::startPair);
        code = label(panel, "", 18); code.setTypeface(android.graphics.Typeface.MONOSPACE); code.setSaveEnabled(false); code.setTag("pairing-code");
        open = button(panel, "Choose encrypted report", () -> {
            Intent i = new Intent(Intent.ACTION_OPEN_DOCUMENT).addCategory(Intent.CATEGORY_OPENABLE).setType("*/*");
            i.putExtra(Intent.EXTRA_LOCAL_ONLY,true);
            try { startActivityForResult(i,PICK); } catch (android.content.ActivityNotFoundException unavailable) { status.setText("This device has no document picker."); }
        });
        revoke = button(panel, "Revoke pairing", () -> {
            clearPair(); status.setText("Pairing revoked. Previously exported files for this pairing cannot be opened here."); refresh();
        });
        status = label(panel, "The report contains capture time, physical memory and power status.", 15);
        results = label(panel, "", 17); results.setSaveEnabled(false);
        field.addAppearanceButton(panel); setContentView(field); refresh();
    }
    private void startPair() {
        new AlertDialog.Builder(this).setTitle("Pair with the attended Windows PC?")
            .setMessage("Anyone who receives this code can make a report for this pairing. Read it directly to the technician. Keep the code separate from the report.")
            .setNegativeButton("Cancel", null).setPositiveButton("Create pairing", (d,w) -> {
                clearPair(); pairing = new DiagnosticHandoff(now(),SystemClock.elapsedRealtime()); results.setText("");
                status.setText("Enter this code in the Windows handoff tool. Expires " + time(pairing.expires()) + ".");
                ui.postDelayed(expire,DiagnosticHandoff.SECONDS*1000L); refresh();
            }).show();
    }
    private static String time(long seconds) { return DateFormat.getDateTimeInstance().format(new Date(seconds * 1000L)); }
    private void clearPair() {
        ui.removeCallbacks(expire);
        if (pending != null) pending.cancel();
        if (pairing != null) pairing.revoke();
        pairing = null;
    }
    private static byte[] readPacket(AssetFileDescriptor fd,CancellationSignal signal) throws Exception {
        long length=fd.getLength();
        if(length>=0 && length!=DiagnosticHandoff.SIZE)throw new EOFException();
        long deadline=SystemClock.elapsedRealtime()+8000;
        byte[] packet=new byte[DiagnosticHandoff.SIZE+1];
        StructPollfd item=new StructPollfd();item.fd=fd.getFileDescriptor();item.events=(short)OsConstants.POLLIN;
        StructPollfd[] poll={item};int offset=0;
        try(InputStream in=fd.createInputStream()){
            while(offset<packet.length) {
                signal.throwIfCanceled();
                long time=deadline-SystemClock.elapsedRealtime();
                if(time<=0)throw new EOFException();
                if(Os.poll(poll,(int)Math.min(time,500))<=0)continue;
                int read=in.read(packet,offset,packet.length-offset);
                if(read<0)break;
                if(read==0)throw new EOFException();
                offset+=read;
            }
        }
        if(offset!=DiagnosticHandoff.SIZE)throw new EOFException();
        return Arrays.copyOf(packet,offset);
    }
    private void refresh() {
        long wall=now(),elapsed=SystemClock.elapsedRealtime();
        boolean valid = pairing != null && pairing.active(wall,elapsed);
        code.setText(valid ? pairing.code(wall,elapsed) : "");
        open.setEnabled(valid && pending == null); revoke.setEnabled(valid); create.setEnabled(pending == null);
    }
    @Override protected void onActivityResult(int req,int result,Intent data) {
        super.onActivityResult(req,result,data);
        if (req!=PICK || result!=RESULT_OK || data==null || data.getData()==null || pairing==null || pending!=null) return;
        Uri uri=data.getData();
        if (!"content".equals(uri.getScheme())) {status.setText("Choose a document from the local file picker."); return;}
        DiagnosticHandoff selected=pairing; CancellationSignal signal=new CancellationSignal();pending=signal;refresh();status.setText("Authenticating selected report…");
        Runnable timeout=signal::cancel;
        ui.postDelayed(timeout,10000);
        disk.execute(() -> {
            DiagnosticHandoff.Snapshot snapshot=null;
            try(AssetFileDescriptor fd=getContentResolver().openAssetFileDescriptor(uri,"r",signal)) {
                if(fd==null) throw new EOFException();
                byte[] packet=readPacket(fd,signal);
                signal.throwIfCanceled();
                snapshot=selected.read(packet,now(),SystemClock.elapsedRealtime());Arrays.fill(packet,(byte)0);
            } catch(Exception ignored) { /* The UI deliberately reports no provider paths or secret material. */ }
            DiagnosticHandoff.Snapshot observed=snapshot;
            ui.post(() -> {
                ui.removeCallbacks(timeout);
                if(isDestroyed() || pending!=signal) return;
                pending=null;
                if(pairing==selected && observed!=null) {
                    clearPair();status.setText("Report authenticated. The pairing has been consumed.");
                    String ac=observed.ac==1?"AC power":observed.ac==0?"Battery power":"Unknown source";
                    String battery=observed.battery==255?"unavailable":observed.battery+"%";
                    results.setText(String.format(Locale.getDefault(),"Captured %s\nPhysical memory %.2f GiB\nAvailable memory %.2f GiB\nMemory load %d%%\nPower source %s\nBattery %s",
                        time(observed.captured),observed.total/1073741824.0,observed.available/1073741824.0,observed.load,ac,battery));
                }else if(pairing==selected){status.setText("Report refused. Check the pairing, file and expiry.");}
                refresh();
            });
        });
    }
    @Override protected void onResume(){super.onResume();if(code!=null)refresh();}
    @Override protected void onDestroy(){clearPair();disk.shutdownNow();super.onDestroy();}
}
