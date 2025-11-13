use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Settings Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Settings Demo",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    fullscreen: bool,
    resolutions: Vec<(u32, u32)>,
    selected_resolution: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            fullscreen: false,
            resolutions: vec![
                (1280, 720),
                (1600, 900),
                (1920, 1080),
                (2560, 1440),
                (3840, 2160),
            ],
            selected_resolution: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Settings");

            // fullscreen toggle
            ui.checkbox(&mut self.fullscreen, "Fullscreen");

            // resolution dropdown
            egui::ComboBox::from_label("Resolution")
                .selected_text(format!(
                    "{}x{}",
                    self.resolutions[self.selected_resolution].0,
                    self.resolutions[self.selected_resolution].1
                ))
                .show_ui(ui, |ui| {
                    for (i, (w, h)) in self.resolutions.iter().enumerate() {
                        ui.selectable_value(
                            &mut self.selected_resolution,
                            i,
                            format!("{w}x{h}"),
                        );
                    }
                });

            if ui.button("Apply").clicked() {
                apply_settings(ctx, self.fullscreen, self.resolutions[self.selected_resolution]);
            }
        });
    }
}

fn apply_settings(ctx: &egui::Context, fullscreen: bool, (width, height): (u32, u32)) {
    use egui::ViewportCommand;

    if fullscreen {
        ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true));
    } else {
        ctx.send_viewport_cmd(ViewportCommand::Fullscreen(false));
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(egui::vec2(
            width as f32,
            height as f32,
        )));
    }
}
