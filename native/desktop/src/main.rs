#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod appearance;
mod backend;

use appearance::{ACCENTS, Appearance, Field, PRESETS};
use backend::{Bridge, CAPABILITIES, field};
use eframe::egui::{self, RichText, Stroke};
use serde_json::Value;
use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum View {
    Appearance,
    Forge,
    Diagnostics,
}
impl View {
    fn parse(args: &[String]) -> Option<Self> {
        let [flag, value] = args else {
            return None;
        };
        if flag != "--view" {
            return None;
        }
        match value.as_str() {
            "appearance" => Some(Self::Appearance),
            "forge" => Some(Self::Forge),
            "diagnostics" => Some(Self::Diagnostics),
            _ => None,
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Appearance => "Appearance Studio",
            Self::Forge => "Capability Forge",
            Self::Diagnostics => "Diagnostics",
        }
    }
}
type Report = Result<Value, String>;
enum Message {
    Discovery(Report),
    Report(String, Report, bool),
}
enum Command {
    ForgeImport(PathBuf),
    ForgeRun(String),
}
struct Consent {
    title: String,
    detail: String,
    action: Command,
}
struct Desktop {
    view: View,
    bridge: Bridge,
    tx: Sender<Message>,
    rx: Receiver<Message>,
    operations: Vec<Value>,
    operation: String,
    operation_input: Option<PathBuf>,
    discovery_error: String,
    forge_file: Option<PathBuf>,
    forge_id: String,
    forge_store: PathBuf,
    working: Option<String>,
    result: Option<(String, Report)>,
    show_result: bool,
    consent: Option<Consent>,
    appearance: Appearance,
    settings_path: PathBuf,
    system_reduced: bool,
    system_contrast: bool,
    field: Field,
    settings_message: String,
    save_due: Option<Instant>,
}
fn main() {
    let Some(view) = View::parse(&std::env::args().skip(1).collect::<Vec<_>>()) else {
        std::process::exit(2)
    };
    let Ok(exe) = std::env::current_exe() else {
        std::process::exit(1)
    };
    let Some(folder) = exe.parent() else {
        std::process::exit(1)
    };
    let bridge = Bridge {
        directory: folder.to_owned(),
    };
    let title = format!("{} · INTA Desktop", view.title());
    let mut icon = Vec::with_capacity(32 * 32 * 4);
    for y in 0..32 {
        for x in 0..32 {
            let across = (6..14).contains(&x) || (18..26).contains(&x);
            let down = (6..14).contains(&y) || (18..26).contains(&y);
            let cut = (20..24).contains(&x) && (20..24).contains(&y);
            icon.extend_from_slice(if across && down && !cut {
                &[157, 178, 255, 255]
            } else {
                &[24, 26, 39, 255]
            });
        }
    }
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(&title)
            .with_inner_size([725., 600.])
            .with_min_inner_size([435., 390.])
            .with_icon(egui::IconData {
                rgba: icon,
                width: 32,
                height: 32,
            }),
        ..Default::default()
    };
    if let Err(error) = eframe::run_native(
        &title,
        opts,
        Box::new(move |cc| Ok(Box::new(Desktop::new(cc, bridge, view)))),
    ) {
        #[cfg(windows)]
        {
            use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MessageBoxW};
            let message: Vec<u16> = format!("The {} view could not open.\n{error}", view.title())
                .encode_utf16()
                .chain(Some(0))
                .collect();
            let caption: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
            unsafe {
                MessageBoxW(
                    std::ptr::null_mut(),
                    message.as_ptr(),
                    caption.as_ptr(),
                    MB_ICONERROR,
                );
            }
        }
        std::process::exit(1);
    }
}
impl Desktop {
    fn new(cc: &eframe::CreationContext<'_>, bridge: Bridge, view: View) -> Self {
        appearance::native_fonts(&cc.egui_ctx);
        let settings_path = backend::prefs_path(&bridge.directory);
        let appearance = Appearance::load(&settings_path);
        let (system_reduced, system_contrast) = appearance::system_preferences();
        let (tx, rx) = mpsc::channel();
        let forge_store = settings_path.with_file_name("forge");
        let x = Self {
            view,
            bridge,
            tx,
            rx,
            operations: vec![],
            operation: "system.memory".into(),
            operation_input: None,
            discovery_error: String::new(),
            forge_file: None,
            forge_id: String::new(),
            forge_store,
            working: None,
            result: None,
            show_result: false,
            consent: None,
            appearance,
            settings_path,
            system_reduced,
            system_contrast,
            field: Field::default(),
            settings_message: String::new(),
            save_due: None,
        };
        x.effective().apply(&cc.egui_ctx);
        if view == View::Diagnostics {
            x.task(&cc.egui_ctx, |b| {
                Message::Discovery(b.json(CAPABILITIES, &["discover"], false))
            });
        }
        x
    }
    fn effective(&self) -> Appearance {
        let mut p = self.appearance.clone();
        p.high_contrast |= self.system_contrast;
        p
    }
    fn task(&self, ctx: &egui::Context, run: impl FnOnce(Bridge) -> Message + Send + 'static) {
        let tx = self.tx.clone();
        let b = self.bridge.clone();
        let c = ctx.clone();
        thread::spawn(move || {
            let m = run(b);
            let _ = tx.send(m);
            c.request_repaint();
        });
    }
    fn collect(&mut self, ctx: &egui::Context) {
        while let Ok(m) = self.rx.try_recv() {
            match m {
                Message::Discovery(r) => match r {
                    Ok(v) => {
                        self.operations = v["operations"].as_array().cloned().unwrap_or_default()
                    }
                    Err(e) => self.discovery_error = e,
                },
                Message::Report(title, result, forge) => {
                    if forge
                        && let Ok(v) = &result
                        && let Some(id) = v
                            .get("id")
                            .or_else(|| v.pointer("/manifest/id"))
                            .and_then(Value::as_str)
                    {
                        self.forge_id = id.into();
                    }
                    self.working = None;
                    self.result = Some((title, result));
                    self.show_result = true;
                }
            }
        }
        if self.save_due.is_some_and(|d| Instant::now() >= d) {
            self.persist();
        }
        if self.save_due.is_some() {
            ctx.request_repaint_after(Duration::from_millis(250));
        }
    }
    fn persist(&mut self) {
        self.save_due = None;
        let bytes = serde_json::to_vec_pretty(&self.appearance).expect("serializable appearance");
        self.settings_message = match backend::save(&self.settings_path, &bytes) {
            Ok(()) => "Saved on this device".into(),
            Err(e) => format!("Could not save appearance: {e}"),
        };
    }
    fn heading(ui: &mut egui::Ui, title: &str) {
        ui.label(RichText::new(title).size(27.).strong());
        ui.add_space(4.);
    }
    fn report(
        &mut self,
        ctx: &egui::Context,
        title: &str,
        name: &'static str,
        args: Vec<String>,
        action: bool,
        forge: bool,
    ) {
        if self.working.is_some() {
            return;
        }
        self.working = Some(title.into());
        self.result = None;
        self.show_result = false;
        let title = title.to_owned();
        self.task(ctx, move |b| {
            let args: Vec<_> = args.iter().map(String::as_str).collect();
            Message::Report(title, b.json(name, &args, action), forge)
        });
    }
    fn execute(&mut self, ctx: &egui::Context, command: Command) {
        let store = self.forge_store.to_string_lossy().into_owned();
        match command {
            Command::ForgeImport(path) => self.report(
                ctx,
                "Import capability",
                CAPABILITIES,
                vec![
                    "forge".into(),
                    "import".into(),
                    path.to_string_lossy().into_owned(),
                    "--store".into(),
                    store,
                ],
                true,
                true,
            ),
            Command::ForgeRun(id) => self.report(
                ctx,
                "Run capability",
                CAPABILITIES,
                vec!["forge".into(), "run".into(), id, "--store".into(), store],
                false,
                true,
            ),
        }
    }

    fn json_text(ui: &mut egui::Ui, value: &Value) {
        let text = serde_json::to_string_pretty(value).unwrap_or_default();
        let shown: String = text.chars().take(16000).collect();
        ui.add(
            egui::Label::new(RichText::new(shown).monospace().size(12.))
                .wrap()
                .selectable(true),
        );
        if text.chars().count() > 16000 {
            ui.label("Showing the first 16,000 characters.");
        }
    }
    fn diagnostics(&mut self, ui: &mut egui::Ui, _p: &Appearance) {
        Self::heading(ui, "Diagnostics");
        if !self.discovery_error.is_empty() {
            ui.label(&self.discovery_error);
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            let old = self.operation.clone();
            egui::ComboBox::from_label("Operation")
                .selected_text(
                    self.operations
                        .iter()
                        .find(|r| r["id"] == self.operation)
                        .map(|r| field(r, "title"))
                        .unwrap_or_else(|| self.operation.clone()),
                )
                .show_ui(ui, |ui| {
                    for op in &self.operations {
                        ui.selectable_value(
                            &mut self.operation,
                            field(op, "id"),
                            field(op, "title"),
                        );
                    }
                });
            if old != self.operation {
                self.operation_input = None;
            }
            if let Some(op) = self.operations.iter().find(|v| v["id"] == self.operation) {
                ui.label(field(op, "description"));
                let needs = op
                    .pointer("/inputSchema/required")
                    .and_then(Value::as_array)
                    .is_some_and(|v| !v.is_empty());
                if needs {
                    if ui.button("Choose input JSON…").clicked() {
                        self.operation_input = rfd::FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .pick_file();
                    }
                    if let Some(path) = &self.operation_input {
                        ui.label(path.display().to_string());
                    }
                }
                let schema = op.clone();
                if ui
                    .add_enabled(
                        self.working.is_none() && (!needs || self.operation_input.is_some()),
                        egui::Button::new("Run operation"),
                    )
                    .clicked()
                {
                    let mut args = vec!["run".into(), self.operation.clone()];
                    if let Some(path) = &self.operation_input {
                        args.extend(["--input".into(), path.to_string_lossy().into_owned()]);
                    }
                    self.report(
                        ui.ctx(),
                        "Diagnostic result",
                        CAPABILITIES,
                        args,
                        false,
                        false,
                    );
                }
                ui.collapsing("Operation schema", |ui| Self::json_text(ui, &schema));
            }
        });
    }
    fn forge(&mut self, ui: &mut egui::Ui, _p: &Appearance) {
        Self::heading(ui, "Capability Forge");
        ui.label("Review a capability manifest before adding it to your local collection.");
        ui.add_space(5.);
        if ui.button("Choose manifest…").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("JSON manifest", &["json"])
                .pick_file()
            {
                self.forge_file = Some(path);
            }
        }
        if let Some(path) = self.forge_file.clone() {
            ui.label(path.display().to_string());
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(self.working.is_none(), egui::Button::new("Verify manifest"))
                    .clicked()
                {
                    self.report(
                        ui.ctx(),
                        "Verify manifest",
                        CAPABILITIES,
                        vec![
                            "forge".into(),
                            "verify".into(),
                            path.to_string_lossy().into_owned(),
                        ],
                        false,
                        true,
                    );
                }
                if ui
                    .add_enabled(
                        self.working.is_none(),
                        egui::Button::new("Import manifest…"),
                    )
                    .clicked()
                {
                    self.consent = Some(Consent {
                        title: "Import capability".into(),
                        detail: format!(
                            "Verify and import {} into {}.",
                            path.display(),
                            self.forge_store.display()
                        ),
                        action: Command::ForgeImport(path),
                    });
                }
            });
        }
        ui.add_space(10.);
        ui.separator();
        ui.label(RichText::new("Run an imported capability").strong());
        ui.add(egui::TextEdit::singleline(&mut self.forge_id).hint_text("Capability ID"));
        if ui
            .add_enabled(
                self.working.is_none() && !self.forge_id.trim().is_empty(),
                egui::Button::new("Review run…"),
            )
            .clicked()
        {
            self.consent = Some(Consent {
                title: "Run imported capability".into(),
                detail: format!(
                    "Run {} from {} using the registered local operations.",
                    self.forge_id,
                    self.forge_store.display()
                ),
                action: Command::ForgeRun(self.forge_id.clone()),
            });
        }
    }
    fn studio(&mut self, ui: &mut egui::Ui, p: &Appearance) {
        Self::heading(ui, "Appearance Studio");
        let prev = self.appearance.clone();
        egui::ScrollArea::vertical().show(ui, |ui| {
            let (rect, _) = ui
                .allocate_exact_size(egui::vec2(ui.available_width(), 110.), egui::Sense::hover());
            ui.painter().rect_filled(rect, 6., p.canvas());
            self.field.draw(ui, rect, p, self.system_reduced);
            ui.painter().rect_stroke(
                rect,
                6.,
                Stroke::new(1., p.line()),
                egui::StrokeKind::Inside,
            );
            ui.add_space(5.);
            ui.label(RichText::new("Optical profile").strong());
            ui.horizontal_wrapped(|ui| {
                for (i, label) in PRESETS.iter().enumerate() {
                    if ui
                        .selectable_label(self.appearance.preset == i as u32, *label)
                        .clicked()
                    {
                        self.appearance.select(i as u32);
                    }
                }
            });
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label("Theme");
                ui.selectable_value(&mut self.appearance.light, false, "Dark");
                ui.selectable_value(&mut self.appearance.light, true, "Light");
                ui.checkbox(&mut self.appearance.high_contrast, "High contrast");
            });
            ui.horizontal_wrapped(|ui| {
                ui.label("Accent");
                for (i, (name, _)) in ACCENTS.iter().enumerate() {
                    ui.selectable_value(&mut self.appearance.accent, i, *name);
                }
            });
            ui.add(egui::Slider::new(&mut self.appearance.density, 15..=100).text("Glyph density"));
            ui.add(
                egui::Slider::new(&mut self.appearance.intensity, 0..=100).text("Field intensity"),
            );
            ui.add(
                egui::Slider::new(&mut self.appearance.strength, 0..=100).text("Ripple strength"),
            );
            ui.add(
                egui::Slider::new(&mut self.appearance.zoom, 80..=150)
                    .suffix("%")
                    .text("Interface scale"),
            );
            ui.checkbox(&mut self.appearance.reduced_motion, "Reduce motion");
            if self.system_reduced || self.system_contrast {
                ui.label("Windows accessibility preferences are active.");
            }
            ui.horizontal_wrapped(|ui| {
                if ui.button("Restore defaults").clicked() {
                    self.appearance = Appearance::default();
                }
                ui.label(
                    RichText::new(&self.settings_message)
                        .size(12.)
                        .color(p.muted()),
                );
            });
        });
        if self.appearance != prev {
            self.effective().apply(ui.ctx());
            self.save_due = Some(Instant::now() + Duration::from_millis(250));
            self.settings_message = "Saving…".into();
            ui.ctx().request_repaint_after(Duration::from_millis(250));
        }
    }
}
impl eframe::App for Desktop {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.collect(&ctx);
        let p = self.effective();
        if self.working.is_some() || self.result.is_some() {
            egui::Panel::bottom("result-status")
                .resizable(false)
                .frame(egui::Frame::NONE.fill(p.surface()).inner_margin(7))
                .show(ui, |ui| {
                    if let Some(w) = &self.working {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(format!("Running {w}…"));
                        });
                    } else if ui.button("View last result").clicked() {
                        self.show_result = true;
                    }
                });
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(p.canvas()).inner_margin(18))
            .show(ui, |ui| match self.view {
                View::Appearance => self.studio(ui, &p),
                View::Forge => {
                    egui::ScrollArea::vertical().show(ui, |ui| self.forge(ui, &p));
                }
                View::Diagnostics => self.diagnostics(ui, &p),
            });
        if let Some(preview) = &self.consent {
            let mut approved = false;
            let mut cancel = false;
            let modal = egui::Modal::new(egui::Id::new("consent")).show(&ctx, |ui| {
                ui.set_max_width(420.);
                ui.heading(&preview.title);
                ui.label(&preview.detail);
                ui.horizontal(|ui| {
                    cancel = ui.button("Cancel").clicked();
                    approved = ui.button("Continue").clicked();
                });
            });
            if approved {
                let act = self.consent.take().expect("open consent").action;
                self.execute(&ctx, act);
            } else if cancel || modal.should_close() {
                self.consent = None;
            }
        }
        if self.show_result
            && let Some((title, report)) = &self.result
        {
            let mut close = false;
            egui::Window::new(title)
                .id(egui::Id::new("result"))
                .collapsible(false)
                .resizable(true)
                .default_width(505.)
                .open(&mut self.show_result)
                .show(&ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(320.)
                        .show(ui, |ui| match report {
                            Ok(v) => Self::json_text(ui, v),
                            Err(e) => {
                                ui.label(e);
                            }
                        });
                    close = ui.button("Close result").clicked();
                });
            if close {
                self.show_result = false;
            }
        }
    }
    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        if self.save_due.is_some() {
            self.persist();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn entry_points() {
        for (word, expected) in [
            ("appearance", View::Appearance),
            ("diagnostics", View::Diagnostics),
            ("forge", View::Forge),
        ] {
            assert_eq!(View::parse(&["--view".into(), word.into()]), Some(expected));
        }
        for words in [
            vec![],
            vec!["--view"],
            vec!["--view", "overview"],
            vec!["--view", "forge", "--run"],
            vec!["inta-tool://open/7zip/launch"],
        ] {
            assert!(
                View::parse(&words.into_iter().map(str::to_owned).collect::<Vec<_>>()).is_none()
            );
        }
    }
}
