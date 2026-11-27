use crate::{copy_to_ventoy, download_release, launch_official, Progress, ISO};
use eframe::egui::{self, Color32, RichText, Vec2};
use inta_rescue::assets::Brands;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
        Arc,
    },
    thread,
};
enum Event {
    Progress(Progress),
    Done(std::result::Result<PathBuf, String>),
}
enum Work {
    Download(PathBuf),
    Copy(PathBuf, PathBuf),
}
pub struct MediaApp {
    capture: inta_rescue::capture::Capture,
    brands: Brands,
    message: String,
    error: bool,
    busy: bool,
    progress: Option<Progress>,
    events: Option<Receiver<Event>>,
    cancel: Arc<AtomicBool>,
    pending_copy: Option<(PathBuf, PathBuf)>,
}
impl MediaApp {
    pub fn new(context: &eframe::CreationContext<'_>) -> Self {
        inta_rescue::ui::configure(&context.egui_ctx);
        Self {
            capture: inta_rescue::capture::Capture::from_args(),
            brands: Brands::default(),
            message: "Ready. Choose a destination to prepare a verified Rescue image.".into(),
            error: false,
            busy: false,
            progress: None,
            events: None,
            cancel: Arc::new(AtomicBool::new(false)),
            pending_copy: None,
        }
    }
    fn start(&mut self, work: Work, context: &egui::Context) {
        self.busy = true;
        self.error = false;
        self.progress = None;
        self.message = "Reading the official release and checking the destination…".into();
        self.cancel = Arc::new(AtomicBool::new(false));
        let cancel = self.cancel.clone();
        let (context, sender, receiver) = {
            let (sender, receiver) = mpsc::channel();
            (context.clone(), sender, receiver)
        };
        self.events = Some(receiver);
        thread::spawn(move || {
            let updates = sender.clone();
            let mut progress = |p| {
                let _ = updates.send(Event::Progress(p));
                context.request_repaint();
            };
            let result = match work {
                Work::Download(folder) => download_release(&folder, &cancel, &mut progress),
                Work::Copy(source, target) => {
                    copy_to_ventoy(&source, &target, &cancel, &mut progress)
                }
            };
            let _ = sender.send(Event::Done(result.map_err(|e| e.to_string())));
            context.request_repaint();
        });
    }
    fn choose_download(&mut self, context: &egui::Context) {
        if let Some(folder) = rfd::FileDialog::new()
            .set_title("Choose where to save the verified INTA Rescue ISO")
            .pick_folder()
        {
            self.start(Work::Download(folder), context);
        }
    }
    fn choose_copy(&mut self) {
        let Some(source) = rfd::FileDialog::new()
            .set_title("Select the INTA ISO next to SHA256SUMS")
            .add_filter("Rescue image", &["iso"])
            .pick_file()
        else {
            return;
        };
        if let Some(target) = rfd::FileDialog::new()
            .set_title("Choose the root of the mounted Ventoy data volume")
            .pick_folder()
        {
            self.pending_copy = Some((source, target));
        }
    }
    fn tool(&mut self, name: &str) {
        let Some(path) = rfd::FileDialog::new()
            .set_title(format!("Choose the verified official {name} executable"))
            .pick_file()
        else {
            return;
        };
        match launch_official(name, &path) {
            Ok(()) => {
                self.message =
                    format!("Opened {name}. Follow its device selection and confirmation steps.");
                self.error = false;
            }
            Err(error) => {
                self.message = error.to_string();
                self.error = true;
            }
        }
    }
}
impl eframe::App for MediaApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        let messages: Vec<_> = self
            .events
            .as_ref()
            .map(|r| r.try_iter().collect())
            .unwrap_or_default();
        for message in messages {
            match message {
                Event::Progress(progress) => {
                    self.message = progress.phase.into();
                    self.progress = Some(progress);
                }
                Event::Done(result) => {
                    self.busy = false;
                    self.events = None;
                    match result {
                        Ok(path) => {
                            self.message = format!("Verified ISO ready: {}", path.display());
                            self.error = false;
                        }
                        Err(error) => {
                            self.message = error;
                            self.error = true;
                        }
                    }
                }
            }
        }
        if self.busy && context.input(|input| input.viewport().close_requested()) {
            context.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.message =
                "A transfer is active. Cancel it or wait for verification before closing.".into();
        }
        egui::TopBottomPanel::bottom("media-status").show(context, |ui| {
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.colored_label(
                    if self.error {
                        Color32::from_rgb(245, 172, 159)
                    } else {
                        Color32::from_rgb(168, 175, 185)
                    },
                    &self.message,
                );
            });
            if let Some(progress) = &self.progress {
                let fraction = if progress.total > 0 {
                    progress.received as f32 / progress.total as f32
                } else {
                    0.0
                };
                ui.add(egui::ProgressBar::new(fraction).text(format!(
                    "{:.1} / {:.1} MiB",
                    progress.received as f64 / 1048576.0,
                    progress.total as f64 / 1048576.0
                )));
            }
            if self.busy && ui.button("Cancel transfer").clicked() {
                self.cancel.store(true, Ordering::Relaxed);
                self.message =
                    "Cancellation requested; waiting for the current read to finish.".into();
            }
            ui.add_space(8.0);
        });
        let mut download = false;
        let mut copy = false;
        let mut tool = None;
        egui::CentralPanel::default().show(context,|ui|{
            ui.add_space(16.0);ui.horizontal(|ui|{if let Some(texture)=self.brands.texture(context,"INTA"){ui.add(egui::Image::new(&texture).fit_to_exact_size(Vec2::splat(36.0)));}ui.vertical(|ui|{ui.label(RichText::new("INTA").size(24.0).strong());ui.label(RichText::new("MEDIA CREATOR").small().color(Color32::from_rgb(139,184,255)));});});
            ui.add_space(20.0);ui.heading(RichText::new("Prepare your recovery media").size(25.0));ui.label("Download a verified Rescue image, then place it on an existing Ventoy USB.");ui.add_space(16.0);
            egui::ScrollArea::vertical().show(ui,|ui|{
                ui.label(RichText::new("1   GET THE RESCUE IMAGE").small().strong().color(Color32::from_rgb(168,175,185)));ui.add_space(8.0);
                egui::Frame::new().fill(Color32::from_rgb(23,25,29)).inner_margin(18).corner_radius(5).show(ui,|ui|{
                    ui.horizontal_top(|ui|{ui.vertical(|ui|{ui.set_width((ui.available_width()-205.0).max(240.0));ui.label(RichText::new("INTA Rescue").size(17.0).strong());ui.label("Checks the official release, verifies every part and the complete SHA-256, then replaces the local image.");});if ui.add_enabled(!self.busy,egui::Button::new("Download / update ISO").min_size(Vec2::new(186.0,38.0))).clicked(){download=true;}});
                    ui.add_space(8.0);ui.hyperlink_to("Official releases ↗","https://github.com/Herbertofury/INTA/releases");
                });
                ui.add_space(20.0);ui.label(RichText::new("2   COPY TO YOUR EXISTING VENTOY USB").small().strong().color(Color32::from_rgb(168,175,185)));ui.add_space(8.0);
                egui::Frame::new().fill(Color32::from_rgb(23,25,29)).inner_margin(18).corner_radius(5).show(ui,|ui|{
                    ui.horizontal_top(|ui|{ui.vertical(|ui|{ui.set_width((ui.available_width()-205.0).max(240.0));ui.label(RichText::new("Verified copy").size(17.0).strong());ui.label("Select the downloaded ISO beside SHA256SUMS, then the mounted Ventoy data volume. The old image stays intact until verification succeeds.");});if ui.add_enabled(!self.busy,egui::Button::new("Copy verified ISO").min_size(Vec2::new(186.0,38.0))).clicked(){copy=true;}});
                });
                ui.add_space(23.0);ui.separator();ui.add_space(12.0);ui.label(RichText::new("USB PREPARATION TOOLS").small().strong().color(Color32::from_rgb(168,175,185)));ui.label("To prepare a new USB, use the publisher's tool and confirm the exact device there.");ui.add_space(8.0);
                for(name,url,description)in[("Ventoy","https://www.ventoy.net/en/download.html","Boot multiple ISO files from one prepared USB."),("Rufus","https://rufus.ie/en/","Create bootable media on Windows.")]{
                    ui.horizontal(|ui|{if let Some(texture)=self.brands.texture(context,name){if ui.add(egui::Image::new(&texture).fit_to_exact_size(Vec2::splat(26.0)).sense(egui::Sense::click())).clicked(){context.open_url(egui::OpenUrl::new_tab(url));}}ui.vertical(|ui|{ui.set_width((ui.available_width()-220.0).max(200.0));ui.hyperlink_to(RichText::new(name).strong(),url);ui.label(RichText::new(description).color(Color32::from_rgb(168,175,185)));});if ui.add_enabled(!self.busy&&(name!="Rufus"||cfg!(windows)),egui::Button::new("Open downloaded tool")).clicked(){tool=Some(name);}});ui.separator();
                }
                ui.add_space(12.0);ui.label(RichText::new("The copy action writes the ISO file only. Formatting and partition changes belong to the selected USB preparation tool.").small().color(Color32::from_rgb(168,175,185)));ui.add_space(12.0);
            });
        });
        if download {
            self.choose_download(context);
        }
        if copy {
            self.choose_copy();
        }
        if let Some(name) = tool {
            self.tool(name);
        }
        if let Some((source, target)) = self.pending_copy.clone() {
            let mut accepted = false;
            let mut canceled = false;
            egui::Window::new("Copy verified image to Ventoy")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(context, |ui| {
                    ui.set_max_width(480.0);
                    ui.label(format!("Install or replace {ISO} on {}?", target.display()));
                    ui.label(RichText::new(format!("Source: {}", source.display())).small());
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button("Verify & copy").clicked() {
                            accepted = true;
                        }
                        if ui.button("Cancel").clicked() {
                            canceled = true;
                        }
                    });
                });
            if accepted || canceled {
                self.pending_copy = None;
            }
            if accepted {
                self.start(Work::Copy(source, target), context);
            }
        }
        self.capture.finish_frame(context);
    }
}
