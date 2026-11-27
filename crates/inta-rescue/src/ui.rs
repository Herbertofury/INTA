use crate::{action, assets::Brands, installed, launch, Action, Group, ACTIONS};
use eframe::egui::{self, Color32, RichText, Vec2};
use std::collections::HashMap;

pub fn configure(context: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = Color32::from_rgb(17, 18, 20);
    visuals.window_fill = Color32::from_rgb(23, 25, 29);
    visuals.extreme_bg_color = Color32::from_rgb(13, 15, 18);
    visuals.faint_bg_color = Color32::from_rgb(27, 30, 35);
    visuals.hyperlink_color = Color32::from_rgb(139, 184, 255);
    visuals.selection.bg_fill = Color32::from_rgb(37, 58, 87);
    visuals.widgets.noninteractive.bg_stroke.color = Color32::from_rgb(48, 53, 61);
    visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(27, 30, 35);
    visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(36, 41, 49);
    context.set_visuals(visuals);
    let mut style = (*context.style()).clone();
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.button_padding = Vec2::new(12.0, 8.0);
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(14.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(14.0));
    context.set_style(style);
}

pub struct RescueApp {
    capture: crate::capture::Capture,
    query: String,
    group: Option<Group>,
    availability: HashMap<&'static str, bool>,
    brands: Brands,
    message: String,
    failure: bool,
    confirmation: Option<&'static str>,
}
impl RescueApp {
    pub fn new(context: &eframe::CreationContext<'_>) -> Self {
        configure(&context.egui_ctx);
        Self {
            capture: crate::capture::Capture::from_args(),
            query: String::new(),
            group: None,
            availability: ACTIONS.iter().map(|a| (a.id, installed(a))).collect(),
            brands: Brands::default(),
            message: "Ready. Open a tool to start; nothing runs in the background.".into(),
            failure: false,
            confirmation: None,
        }
    }
    fn run(&mut self, action: &Action, confirmed: bool) {
        if action.online_confirmation && !confirmed {
            self.confirmation = Some(action.id);
            return;
        }
        let folder = if action.id == "scan" {
            match rfd::FileDialog::new()
                .set_title("Choose a directory to scan with ClamAV")
                .pick_folder()
            {
                Some(folder) => Some(folder),
                None => return,
            }
        } else {
            None
        };
        match launch(action, folder.as_deref(), confirmed) {
            Ok(pid) => {
                self.message = format!(
                    "Opened {} (process {pid}). Review progress and results in the tool.",
                    action.title
                );
                self.failure = false;
            }
            Err(error) => {
                self.message = format!("Could not open {}: {error}", action.title);
                self.failure = true;
            }
        }
    }
}
impl eframe::App for RescueApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        let mut requested = None;
        egui::TopBottomPanel::bottom("status").show(context, |ui| {
            ui.add_space(5.0);
            ui.horizontal_wrapped(|ui| {
                ui.colored_label(
                    if self.failure {
                        Color32::from_rgb(245, 172, 159)
                    } else {
                        Color32::from_rgb(168, 175, 185)
                    },
                    &self.message,
                );
            });
            ui.add_space(5.0);
        });
        egui::SidePanel::left("navigation").resizable(false).default_width(184.0).show(context, |ui| {
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                if let Some(icon) = self.brands.texture(context,"INTA") { ui.add(egui::Image::new(&icon).fit_to_exact_size(Vec2::splat(28.0))); }
                ui.label(RichText::new("INTA").size(22.0).strong());
            });
            ui.label(RichText::new("RESCUE").small().color(Color32::from_rgb(168,175,185)));
            ui.add_space(23.0);
            ui.label(RichText::new("LOCAL WORKSPACE").small().color(Color32::from_rgb(147,156,169)));
            if ui.selectable_label(self.group.is_none(), "All tools  ·  14").clicked() { self.group = None; }
            for group in [Group::Disks,Group::System,Group::Ai] {
                if ui.selectable_label(self.group == Some(group), group.label()).clicked() { self.group = Some(group); }
            }
            ui.add_space(25.0); ui.separator(); ui.add_space(10.0);
            ui.hyperlink_to("Full tool library ↗", "https://herbertofury.github.io/INTA/#tools");
            ui.hyperlink_to("Recovery guides ↗", "https://herbertofury.github.io/INTA/#playbooks");
            ui.hyperlink_to("Release & source ↗", "https://github.com/Herbertofury/INTA/releases");
            ui.add_space(16.0);
            ui.label(RichText::new("Live-session data is temporary. Export evidence to a separate destination.").small().color(Color32::from_rgb(168,175,185)));
        });
        egui::CentralPanel::default().show(context, |ui| {
            ui.add_space(12.0);
            ui.label(
                RichText::new("RECOVERY WORKSPACE")
                    .small()
                    .color(Color32::from_rgb(139, 184, 255)),
            );
            ui.heading(RichText::new("Recovery tools").size(26.0));
            ui.label("Inspect hardware, recover data and work from a clean live system.");
            ui.add_space(13.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text("Find a tool or task…")
                        .desired_width((ui.available_width() - 150.0).max(180.0)),
                );
                if ui.button("Refresh availability").clicked() {
                    self.availability = ACTIONS.iter().map(|a| (a.id, installed(a))).collect();
                }
            });
            ui.add_space(8.0);
            let query = self.query.to_lowercase();
            let matches: Vec<_> = ACTIONS
                .iter()
                .filter(|a| {
                    (self.group.is_none() || self.group == Some(a.group))
                        && format!("{} {} {}", a.title, a.purpose, a.group.label())
                            .to_lowercase()
                            .contains(&query)
                })
                .collect();
            ui.label(
                RichText::new(format!(
                    "{} tools  ·  Official sources stay separate from local launch actions",
                    matches.len()
                ))
                .small()
                .color(Color32::from_rgb(168, 175, 185)),
            );
            ui.add_space(5.0);
            ui.separator();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if matches.is_empty() {
                        ui.label("No tool matches this search.");
                        if ui.button("Clear search & category").clicked() {
                            self.query.clear();
                            self.group = None;
                        }
                    }
                    let mut previous = None;
                    for entry in matches {
                        if previous != Some(entry.group) {
                            ui.add_space(16.0);
                            ui.label(RichText::new(entry.group.label()).size(16.0).strong());
                            ui.add_space(5.0);
                            previous = Some(entry.group);
                        }
                        egui::Frame::new()
                            .inner_margin(egui::Margin::symmetric(0, 8))
                            .show(ui, |ui| {
                                ui.horizontal_top(|ui| {
                                    let available =
                                        self.availability.get(entry.id).copied().unwrap_or(false);
                                    if let Some(texture) = self.brands.texture(context, entry.brand)
                                    {
                                        let image = egui::Image::new(&texture)
                                            .fit_to_exact_size(Vec2::splat(28.0))
                                            .sense(egui::Sense::click());
                                        if ui.add(image).on_hover_text("Official source").clicked()
                                        {
                                            if let Some(url) = self.brands.url(entry.brand) {
                                                context.open_url(egui::OpenUrl::new_tab(url));
                                            }
                                        }
                                    } else {
                                        ui.allocate_space(Vec2::splat(28.0));
                                    }
                                    ui.vertical(|ui| {
                                        ui.set_width((ui.available_width() - 155.0).max(200.0));
                                        if let Some(url) = self.brands.url(entry.brand) {
                                            ui.hyperlink_to(
                                                RichText::new(entry.title).strong().size(16.0),
                                                url,
                                            );
                                        } else {
                                            ui.label(
                                                RichText::new(entry.title).strong().size(16.0),
                                            );
                                        }
                                        ui.label(
                                            RichText::new(entry.purpose)
                                                .color(Color32::from_rgb(168, 175, 185)),
                                        );
                                    });
                                    ui.vertical(|ui| {
                                        if ui
                                            .add_enabled(
                                                available,
                                                egui::Button::new(entry.label)
                                                    .min_size(Vec2::new(140.0, 34.0)),
                                            )
                                            .clicked()
                                        {
                                            requested = Some(entry.id);
                                        }
                                        ui.label(
                                            RichText::new(if available {
                                                "Installed locally"
                                            } else {
                                                "Not installed here"
                                            })
                                            .small()
                                            .color(Color32::from_rgb(147, 156, 169)),
                                        );
                                    });
                                });
                            });
                        ui.separator();
                    }
                    ui.add_space(18.0);
                });
        });
        if let Some(id) = requested {
            if let Some(entry) = action(id) {
                self.run(entry, false);
            }
        }
        if let Some(id) = self.confirmation {
            let mut accepted = false;
            let mut canceled = false;
            egui::Window::new("Online AI session").collapsible(false).resizable(false).anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO).show(context,|ui| {
                ui.set_max_width(430.0);
                ui.label("The provider receives what you share. Review the content for customer data or secrets. The client opens in a private working folder within the live account.");
                ui.add_space(10.0); ui.horizontal(|ui| { if ui.button("Start session").clicked(){accepted=true;} if ui.button("Cancel").clicked(){canceled=true;} });
            });
            if accepted || canceled {
                self.confirmation = None;
            }
            if accepted {
                if let Some(entry) = action(id) {
                    self.run(entry, true);
                }
            }
        }
        self.capture.finish_frame(context);
    }
}
