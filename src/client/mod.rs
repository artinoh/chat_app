pub mod gui;
pub mod client;

use crate::common::config::ServerSettings;

pub fn run_client(settings: &ServerSettings) -> Result<(), Box<dyn std::error::Error>> {
    let app = crate::client::gui::ChatApp::default_with_server(settings);
    let native_options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native("Chat App", native_options, Box::new(|_cc| Ok(Box::new(app)))) {
        eprintln!("Error running app: {}", e);
    }
    Ok(())
}
