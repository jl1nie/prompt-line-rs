//! prompt-line-rs: A floating text input tool
//!
//! Quick launch with global hotkey (Alt+Space), type text, and quick paste (Ctrl+Enter).

use eframe::egui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

mod hotkey;

fn main() -> eframe::Result<()> {
    // Shared state for hotkey toggle
    let toggle_flag = Arc::new(AtomicBool::new(false));
    let toggle_flag_clone = toggle_flag.clone();

    // Start hotkey listener in background thread
    std::thread::spawn(move || {
        if let Err(e) = hotkey::listen_hotkey(toggle_flag_clone) {
            eprintln!("Hotkey listener error: {}", e);
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 500.0])
            .with_always_on_top()
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "prompt-line-rs",
        options,
        Box::new(move |cc| {
            // Set large font size for 4K
            let base_font_size = 28.0;
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::proportional(base_font_size),
            );
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::proportional(base_font_size),
            );
            style.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::proportional(base_font_size * 1.4),
            );
            style.text_styles.insert(
                egui::TextStyle::Monospace,
                egui::FontId::monospace(base_font_size),
            );
            style.spacing.item_spacing = egui::vec2(16.0, 12.0);
            style.spacing.button_padding = egui::vec2(16.0, 8.0);
            cc.egui_ctx.set_style(style);

            Ok(Box::new(PromptLineApp {
                text: String::new(),
                toggle_flag,
                visible: true,
            }))
        }),
    )
}

struct PromptLineApp {
    text: String,
    toggle_flag: Arc<AtomicBool>,
    visible: bool,
}

impl eframe::App for PromptLineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for hotkey toggle
        if self.toggle_flag.swap(false, Ordering::SeqCst) {
            self.visible = !self.visible;
            println!("Toggle! visible: {}", self.visible);
        }

        // Request periodic repaint
        ctx.request_repaint_after(Duration::from_millis(50));

        if self.visible {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("prompt-line-rs");
                ui.add_space(16.0);

                let text_edit = egui::TextEdit::multiline(&mut self.text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(10)
                    .font(egui::TextStyle::Body)
                    .hint_text("Type your text here... (Ctrl+Enter to paste)");

                ui.add(text_edit);

                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.button("Paste (Ctrl+Enter)").clicked() {
                        println!("Paste: {}", self.text);
                    }
                    if ui.button("Close (Esc)").clicked() {
                        self.visible = false;
                    }
                });
            });

            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.visible = false;
            }
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }
    }
}
