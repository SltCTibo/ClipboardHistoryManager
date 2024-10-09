use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let app_state = Arc::new(Mutex::new(MyEguiApp::default()));

    // Start the clipboard watcher in a background thread
    let app_state_clone = Arc::clone(&app_state);
    thread::spawn(move || {
        let mut clipboard = ClipboardContext::new().unwrap();
        let mut previous_content = String::new();

        loop {
            let current_content = clipboard.get_contents().unwrap_or_else(|_| String::new());
            if current_content != previous_content {
                if let Ok(mut app) = app_state_clone.lock() {
                    app.clipboard_content = current_content.clone();
                    if previous_content.len() != 0 {
                        app.previous_contents.push(previous_content.clone());
                    }
                }
                previous_content = current_content;
            }
            // Check every 500 milliseconds for clipboard changes
            thread::sleep(Duration::from_millis(500));
        }
    });

    eframe::run_native(
        "Clipboard Viewer",
        native_options,
        Box::new(move |_cc| Ok(Box::new(MyEguiAppWrapper { app_state: Arc::clone(&app_state) }))),
    );
}

#[derive(Default, Clone)]ck
struct MyEguiApp {
    clipboard_content: String,
    previous_contents: Vec<String>
}

struct MyEguiAppWrapper {
    app_state: Arc<Mutex<MyEguiApp>>,
}

impl eframe::App for MyEguiAppWrapper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let clipboard_content = {
            if let Ok(app) = self.app_state.lock() {
                app.clipboard_content.clone()
            } else {
                "Error accessing clipboard content".to_string()
            }
        };

        let previous_content = {
            if let Ok(app) = self.app_state.lock() {
                app.previous_contents.clone()
            } else {
                vec!["Previous contents from the clipboard:".to_string()]
            }
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            for cont in previous_content {
                ui.horizontal(|ui| {
                    ui.label(&cont);
                    if ui.button("Copy").clicked() {
                        let mut clipboard = ClipboardContext::new().unwrap();
                        clipboard.set_contents(cont.clone()).unwrap();
                    }
                });
            }
            ui.label("\nActual clipboard content:\n");
            ui.label(&clipboard_content);
        });

        // Ensure the UI updates regularly
        ctx.request_repaint();
    }
}
