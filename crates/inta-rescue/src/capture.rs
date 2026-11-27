use eframe::egui;
use std::path::PathBuf;

/// One-shot rendered screenshot for CI. In ordinary use this does nothing and schedules no work.
#[derive(Default)]
pub struct Capture {
    path: Option<PathBuf>,
    requested: bool,
}
impl Capture {
    pub fn from_args() -> Self {
        let args: Vec<_> = std::env::args_os().collect();
        let path = args
            .windows(2)
            .find(|pair| pair[0] == "--screenshot")
            .map(|pair| PathBuf::from(&pair[1]))
            .or_else(|| std::env::var_os("INTA_SMOKE_SCREENSHOT").map(PathBuf::from));
        Self {
            path,
            requested: false,
        }
    }
    pub fn finish_frame(&mut self, context: &egui::Context) {
        let Some(path) = self.path.clone() else {
            return;
        };
        let image = context.input(|input| {
            input.events.iter().find_map(|event| match event {
                egui::Event::Screenshot { image, .. } => Some(image.clone()),
                _ => None,
            })
        });
        if let Some(image) = image {
            let rgba: Vec<u8> = image
                .pixels
                .iter()
                .flat_map(|pixel| pixel.to_array())
                .collect();
            if let Some(parent) = path.parent() {
                if let Err(error) = std::fs::create_dir_all(parent) {
                    eprintln!("Screenshot directory: {error}");
                    std::process::exit(1);
                }
            }
            let image =
                image::RgbaImage::from_raw(image.size[0] as u32, image.size[1] as u32, rgba)
                    .expect("RGBA dimensions");
            if let Err(error) = image.save(&path) {
                eprintln!("Could not save screenshot: {error}");
                std::process::exit(1);
            }
            println!("Rendered native screenshot: {}", path.display());
            self.path = None;
            context.send_viewport_cmd(egui::ViewportCommand::Close);
        } else if !self.requested {
            self.requested = true;
            context.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
        }
    }
}
