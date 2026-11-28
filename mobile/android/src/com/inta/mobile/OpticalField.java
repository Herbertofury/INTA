package com.inta.mobile;

import android.animation.ValueAnimator;
import android.app.Activity;
import android.app.AlertDialog;
import android.app.UiModeManager;
import android.content.Context;
import android.content.SharedPreferences;
import android.graphics.Canvas;
import android.graphics.Matrix;
import android.graphics.Paint;
import android.graphics.RadialGradient;
import android.graphics.Shader;
import android.graphics.Typeface;
import android.os.Build;
import android.os.PowerManager;
import android.os.SystemClock;
import android.provider.Settings;
import android.view.Gravity;
import android.view.MotionEvent;
import android.widget.Button;
import android.widget.FrameLayout;
import android.widget.LinearLayout;

/** Paints behind the activity's normal semantic views; touch delivery is unchanged. */
final class OpticalField extends FrameLayout {
    private static final String[] PROFILES = {"Full Prism", "Professional", "Deep Focus", "Silent Eco", "Classic"};
    private static final int SETTLE_MS = 1450;
    private static final String[] GLYPHS = new String[128];
    static final boolean NATIVE;
    static {
        for (int i = 0; i < GLYPHS.length; i++) GLYPHS[i] = Character.toString((char)i);
        boolean loaded;
        try { System.loadLibrary("inta_optics"); loaded = true; }
        catch (UnsatisfiedLinkError | SecurityException unavailable) { loaded = false; }
        NATIVE = loaded;
    }
    private static native int cell(int col, int row, int cols, int rows, int profile, long newest, long previous);

    private final Paint ink = new Paint(Paint.ANTI_ALIAS_FLAG);
    private final Paint blueLight = new Paint();
    private final Paint violetLight = new Paint();
    private final SharedPreferences settings;
    private final float pitch;
    private int profile;
    private long point, previous, time = -SETTLE_MS, previousTime = -SETTLE_MS;
    private boolean reduced, contrast;

    OpticalField(Activity activity) {
        super(activity);
        settings = activity.getSharedPreferences("optics-v1", Context.MODE_PRIVATE);
        profile = settings.getInt("profile", 0);
        if (profile < 0 || profile >= PROFILES.length) profile = 0;
        pitch = 13f * getResources().getDisplayMetrics().density;
        ink.setTypeface(Typeface.MONOSPACE);
        ink.setTextSize(pitch * .70f);
        setWillNotDraw(false);
        refreshSystemSettings();
    }

    private void refreshSystemSettings() {
        PowerManager power = getContext().getSystemService(PowerManager.class);
        UiModeManager mode = getContext().getSystemService(UiModeManager.class);
        contrast = Settings.Secure.getInt(getContext().getContentResolver(), "high_text_contrast_enabled", 0) != 0
                || (Build.VERSION.SDK_INT >= 34 && mode != null && mode.getContrast() > .5f);
        reduced = !ValueAnimator.areAnimatorsEnabled() || (power != null && power.isPowerSaveMode());
    }

    void addAppearanceButton(LinearLayout parent) {
        Button button = new Button(getContext(), null, android.R.attr.borderlessButtonStyle);
        button.setAllCaps(false);
        button.setText("Appearance");
        button.setTextColor(0xff9fc7ed);
        button.setGravity(Gravity.START | Gravity.CENTER_VERTICAL);
        button.setMinHeight(Math.round(pitch * 48f / 13f));
        button.setOnClickListener(view -> {
            AlertDialog.Builder picker = new AlertDialog.Builder(getContext())
                    .setTitle("Appearance")
                    .setSingleChoiceItems(PROFILES, profile, (dialog, choice) -> {
                        profile = choice;
                        settings.edit().putInt("profile", choice).apply();
                        refreshSystemSettings();
                        invalidate();
                        dialog.dismiss();
                    }).setNegativeButton("Close", null);
            if (!NATIVE) picker.setTitle("Appearance · compatibility mode");
            picker.show();
        });
        parent.addView(button, new LinearLayout.LayoutParams(-2,-2));
    }

    @Override public void onWindowFocusChanged(boolean focused) {
        super.onWindowFocusChanged(focused);
        if (focused) {
            profile = settings.getInt("profile", 0);
            if (profile < 0 || profile >= PROFILES.length) profile = 0;
            refreshSystemSettings();
        }
        time = previousTime = -SETTLE_MS;
        invalidate();
    }

    @Override public boolean dispatchTouchEvent(MotionEvent event) {
        int kind = event.getActionMasked();
        long now = SystemClock.uptimeMillis();
        if (NATIVE && !contrast && !reduced && profile < 2 && getWidth() > 0 && getHeight() > 0
                && (kind == MotionEvent.ACTION_DOWN || (kind == MotionEvent.ACTION_MOVE && now-time >= 70))) {
            previous = point;
            previousTime = time;
            int x = Math.min(65535, Math.max(0, Math.round(65535f * event.getX() / getWidth())));
            int y = Math.min(65535, Math.max(0, Math.round(65535f * event.getY() / getHeight())));
            point = x | ((long)y << 16) | ((kind == MotionEvent.ACTION_DOWN ? 255L : 138L) << 48);
            time = now;
            postInvalidateOnAnimation();
        }
        return super.dispatchTouchEvent(event);
    }

    private static long impulse(long value, long age) {
        return age >= 0 && age < SETTLE_MS ? value | (age << 32) : 0;
    }

    @Override protected void onSizeChanged(int w, int h, int oldWidth, int oldHeight) {
        super.onSizeChanged(w, h, oldWidth, oldHeight);
        if (w == 0 || h == 0) return;
        placeLight(blueLight, 0xff34566f, w*.77f, h*.25f, w*.67f, h*.58f);
        placeLight(violetLight, 0xff544365, w*.24f, h*.64f, w*.52f, h*.64f);
    }

    private static void placeLight(Paint paint, int color, float x, float y, float rx, float ry) {
        RadialGradient gradient = new RadialGradient(0, 0, 1, color, color & 0xffffff, Shader.TileMode.CLAMP);
        Matrix matrix = new Matrix();
        matrix.setScale(rx, ry);
        matrix.postTranslate(x, y);
        gradient.setLocalMatrix(matrix);
        paint.setShader(gradient);
    }

    @Override protected void onDraw(Canvas canvas) {
        super.onDraw(canvas);
        canvas.drawColor(contrast ? 0xff000000 : 0xff0b1019);
        if (!NATIVE || contrast || profile >= 4 || getWidth() == 0 || getHeight() == 0) return;
        int light = profile == 0 ? 112 : profile == 1 ? 77 : profile == 2 ? 47 : 24;
        blueLight.setAlpha(light);
        violetLight.setAlpha(light);
        canvas.drawRect(0,0,getWidth(),getHeight(),blueLight);
        canvas.drawRect(0,0,getWidth(),getHeight(),violetLight);
        long now = SystemClock.uptimeMillis();
        long a = reduced ? 0 : impulse(point, now-time);
        long b = reduced ? 0 : impulse(previous, now-previousTime);
        int cols = (int)Math.ceil(getWidth()/pitch);
        int rows = (int)Math.ceil(getHeight()/(pitch*1.25f));
        for (int y = 0; y < rows; y++) {
            for (int x = 0; x < cols; x++) {
                int packed = cell(x,y,cols,rows,profile,a,b);
                if (packed == 0) continue;
                ink.setColor(0xff000000 | (packed & 0xffffff));
                canvas.drawText(GLYPHS[packed >>> 24], (x+.16f)*pitch, (y+.75f)*pitch*1.25f, ink);
            }
        }
        if (profile < 2 && (a != 0 || b != 0) && isShown() && hasWindowFocus()) postInvalidateOnAnimation();
    }
}
