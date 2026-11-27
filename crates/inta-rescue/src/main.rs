fn main() -> eframe::Result<()> {
    if std::env::args().any(|arg| arg == "--self-test") {
        match inta_rescue::assets::self_test() {
            Ok(count) => {
                println!(
                    "Verified {count} bundled brand images and {} native Rust launch actions",
                    inta_rescue::ACTIONS.len()
                );
                return Ok(());
            }
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
    }
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1140.0, 800.0])
            .with_min_inner_size([800.0, 540.0]),
        ..Default::default()
    };
    eframe::run_native(
        "INTA Rescue",
        options,
        Box::new(|context| Ok(Box::new(inta_rescue::ui::RescueApp::new(context)))),
    )
}
