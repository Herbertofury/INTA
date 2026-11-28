use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, Stroke};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    time::{Duration, Instant},
};

pub const PRESETS: [&str; 5] = [
    "Full Prism",
    "Professional",
    "Deep Focus",
    "Silent Eco",
    "Classic",
];
pub const ACCENTS: [(&str, [u8; 3]); 4] = [
    ("Iris", [156, 164, 255]),
    ("Azure", [107, 180, 246]),
    ("Violet", [193, 155, 246]),
    ("Amber", [242, 185, 107]),
];

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Appearance {
    pub preset: u32,
    pub light: bool,
    pub high_contrast: bool,
    pub reduced_motion: bool,
    pub density: u8,
    pub intensity: u8,
    pub strength: u8,
    pub zoom: u8,
    pub accent: usize,
}
impl Default for Appearance {
    fn default() -> Self {
        Self {
            preset: 0,
            light: false,
            high_contrast: false,
            reduced_motion: false,
            density: 60,
            intensity: 86,
            strength: 85,
            zoom: 100,
            accent: 0,
        }
    }
}
impl Appearance {
    pub fn load(path: &Path) -> Self {
        let mut p: Self = fs::read(path)
            .ok()
            .filter(|v| v.len() < 8192)
            .and_then(|v| serde_json::from_slice(&v).ok())
            .unwrap_or_default();
        p.preset = p.preset.min(4);
        p.accent = p.accent.min(3);
        p.zoom = p.zoom.clamp(80, 150);
        p.strength = p.strength.min(100);
        p.density = p.density.clamp(15, 100);
        p.intensity = p.intensity.min(100);
        p
    }
    pub fn select(&mut self, index: u32) {
        self.preset = index;
        let values = [
            (60, 86, 85),
            (45, 27, 46),
            (38, 25, 0),
            (24, 19, 0),
            (30, 0, 0),
        ];
        let (d, i, s) = values[index as usize];
        self.density = d;
        self.intensity = i;
        self.strength = s;
    }
    pub fn accent(&self) -> Color32 {
        let [r, g, b] = ACCENTS[self.accent].1;
        let c = Color32::from_rgb(r, g, b);
        if self.light {
            c.gamma_multiply(0.57)
        } else {
            c
        }
    }
    pub fn canvas(&self) -> Color32 {
        if self.high_contrast {
            if self.light {
                Color32::WHITE
            } else {
                Color32::BLACK
            }
        } else if self.light {
            Color32::from_rgb(244, 245, 249)
        } else {
            Color32::from_rgb(16, 17, 25)
        }
    }
    pub fn surface(&self) -> Color32 {
        if self.high_contrast {
            self.canvas()
        } else if self.light {
            Color32::WHITE
        } else {
            Color32::from_rgb(26, 28, 39)
        }
    }
    pub fn line(&self) -> Color32 {
        if self.high_contrast {
            if self.light {
                Color32::BLACK
            } else {
                Color32::WHITE
            }
        } else if self.light {
            Color32::from_gray(205)
        } else {
            Color32::from_rgb(53, 55, 71)
        }
    }
    pub fn muted(&self) -> Color32 {
        if self.high_contrast {
            if self.light {
                Color32::BLACK
            } else {
                Color32::WHITE
            }
        } else if self.light {
            Color32::from_gray(75)
        } else {
            Color32::from_rgb(179, 182, 201)
        }
    }
    pub fn apply(&self, ctx: &egui::Context) {
        let mut v = if self.light {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        };
        v.panel_fill = self.canvas();
        v.window_fill = self.surface();
        v.faint_bg_color = self.surface();
        v.extreme_bg_color = self.canvas();
        v.widgets.noninteractive.bg_stroke = Stroke::new(1., self.line());
        v.widgets.inactive.weak_bg_fill = self.surface();
        v.selection.bg_fill = self
            .accent()
            .gamma_multiply(if self.light { 0.65 } else { 0.36 });
        v.selection.stroke = Stroke::new(1.5, self.accent());
        if self.high_contrast {
            v.override_text_color = Some(if self.light {
                Color32::BLACK
            } else {
                Color32::WHITE
            });
            v.widgets.inactive.bg_stroke = Stroke::new(1., self.line());
        }
        ctx.set_visuals(v);
        ctx.set_zoom_factor(self.zoom as f32 / 100.);
        ctx.global_style_mut(|s| {
            s.spacing.item_spacing = egui::vec2(8., 8.);
            s.spacing.button_padding = egui::vec2(10., 7.);
            s.spacing.interact_size = egui::vec2(40., 28.);
            s.text_styles.insert(
                egui::TextStyle::Body,
                FontId::new(15., FontFamily::Proportional),
            );
            s.text_styles.insert(
                egui::TextStyle::Button,
                FontId::new(15., FontFamily::Proportional),
            );
        });
    }
}

pub fn system_preferences() -> (bool, bool) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            SPI_GETCLIENTAREAANIMATION, SPI_GETHIGHCONTRAST, SystemParametersInfoW,
        };
        #[repr(C)]
        struct Contrast {
            size: u32,
            flags: u32,
            scheme: *mut u16,
        }
        let mut anim = 1i32;
        let mut contrast = Contrast {
            size: std::mem::size_of::<Contrast>() as u32,
            flags: 0,
            scheme: std::ptr::null_mut(),
        };
        // These read-only SPI requests write to the correctly sized caller-owned structures.
        unsafe {
            SystemParametersInfoW(
                SPI_GETCLIENTAREAANIMATION,
                0,
                (&mut anim as *mut i32).cast(),
                0,
            );
            SystemParametersInfoW(
                SPI_GETHIGHCONTRAST,
                contrast.size,
                (&mut contrast as *mut Contrast).cast(),
                0,
            );
        }
        (anim == 0, contrast.flags & 1 != 0)
    }
    #[cfg(not(windows))]
    {
        (false, false)
    }
}

pub fn native_fonts(ctx: &egui::Context) {
    #[cfg(windows)]
    {
        let Some(root) = std::env::var_os("SystemRoot") else {
            return;
        };
        let root = Path::new(&root).join("Fonts");
        let mut fonts = egui::FontDefinitions::default();
        for (name, file, family) in [
            ("Segoe UI", "segoeui.ttf", FontFamily::Proportional),
            ("Consolas", "consola.ttf", FontFamily::Monospace),
        ] {
            let p = root.join(file);
            if let Ok(b) = fs::read(p)
                && b.len() < 12_000_000
            {
                fonts
                    .font_data
                    .insert(name.into(), egui::FontData::from_owned(b).into());
                fonts
                    .families
                    .entry(family)
                    .or_default()
                    .insert(0, name.into());
            }
        }
        ctx.set_fonts(fonts);
    }
}

pub struct Field {
    waves: Vec<(Pos2, Instant, u8)>,
    pointer: Option<Pos2>,
    last_move: Instant,
}
impl Default for Field {
    fn default() -> Self {
        Self {
            waves: Vec::new(),
            pointer: None,
            last_move: Instant::now(),
        }
    }
}
impl Field {
    pub fn draw(&mut self, ui: &egui::Ui, rect: Rect, settings: &Appearance, system_reduced: bool) {
        if rect.width() < 2.
            || rect.height() < 2.
            || settings.preset == 4
            || settings.high_contrast
            || settings.intensity == 0
        {
            return;
        }
        let now = Instant::now();
        let h = (32. - settings.density as f32 * 0.15).clamp(15., 32.);
        let w = h * 0.87;
        let cols = (rect.width() / w).floor().clamp(2., 180.) as u32;
        let rows = (rect.height() / h).floor().clamp(2., 120.) as u32;
        let moving = !settings.reduced_motion
            && !system_reduced
            && settings.preset < 2
            && settings.strength > 0
            && ui.input(|i| i.focused);
        self.waves.retain(|(_, time, _)| {
            now.duration_since(*time).as_millis() < u128::from(inta_optics::SETTLE_MS)
        });
        let (pointer, pressed) = ui.input(|i| (i.pointer.hover_pos(), i.pointer.any_pressed()));
        if moving {
            if let Some(p) = pointer.filter(|p| rect.contains(*p))
                && (pressed
                    || (self.pointer.is_none_or(|a| a.distance(p) > 12.)
                        && self.last_move.elapsed() >= Duration::from_millis(115)))
            {
                self.pointer = Some(p);
                self.last_move = now;
                self.waves
                    .insert(0, (p, now, if pressed { 255 } else { 165 }));
                self.waves.truncate(2);
            }
        } else {
            self.waves.clear();
        }
        let wave = |index: usize| {
            self.waves.get(index).map_or(0, |(p, t, s)| {
                let x = ((p.x - rect.left()) / rect.width()).clamp(0., 1.);
                let y = ((p.y - rect.top()) / rect.height()).clamp(0., 1.);
                let strength = (*s as u16 * settings.strength as u16 / 100) as u8;
                inta_optics::ripple(
                    (x * 65535.) as u16,
                    (y * 65535.) as u16,
                    t.elapsed().as_millis().min(65535) as u16,
                    strength,
                )
            })
        };
        let (a, b) = (wave(0), wave(1));
        let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
        let mut symbols = BTreeMap::new();
        let base = settings.canvas();
        for row in 0..rows {
            for col in 0..cols {
                let sample = inta_optics::sample_cell(col, row, cols, rows, settings.preset, a, b);
                let ch = ((sample >> 24) as u8) as char;
                if ch.is_ascii_whitespace() || ch.is_control() {
                    continue;
                }
                let alpha = settings.intensity as f32 / 100.;
                let mix = |v: u8, background: u8| {
                    ((v as f32 * alpha) + (background as f32 * (1. - alpha))) as u8
                };
                let color = Color32::from_rgb(
                    mix((sample >> 16) as u8, base.r()),
                    mix((sample >> 8) as u8, base.g()),
                    mix(sample as u8, base.b()),
                );
                let glyph = symbols.entry(ch).or_insert_with(|| {
                    painter.layout_no_wrap(
                        ch.to_string(),
                        FontId::monospace(h * 0.62),
                        Color32::WHITE,
                    )
                });
                let pos = egui::pos2(
                    rect.left() + (col as f32 + 0.42) * w,
                    rect.top() + (row as f32 + 0.2) * h,
                );
                painter.galley_with_override_text_color(pos, glyph.clone(), color);
            }
        }
        if moving && (a != 0 || b != 0) {
            ui.ctx().request_repaint_after(Duration::from_millis(34));
        }
    }
}
