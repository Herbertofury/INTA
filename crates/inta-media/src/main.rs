fn main() -> eframe::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).is_some_and(|s| s == "--verify-release") {
        if args.len() != 4 {
            eprintln!("Usage: inta-media --verify-release <directory> <original-ISO-sha256>");
            std::process::exit(2);
        }
        match inta_media::verify_local_release_directory(std::path::Path::new(&args[2]), &args[3]) {
            Ok(report) => {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
                return Ok(());
            }
            Err(error) => {
                eprintln!("Release verification failed: {error}");
                std::process::exit(1);
            }
        }
    }
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([940.0, 700.0])
            .with_min_inner_size([720.0, 540.0]),
        ..Default::default()
    };
    eframe::run_native(
        "INTA Media Creator",
        options,
        Box::new(|context| Ok(Box::new(inta_media::ui::MediaApp::new(context)))),
    )
}
