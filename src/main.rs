use eframe::egui;

#[derive(Clone, Copy, Debug, PartialEq)]
enum OpenSimplexVariant {
    Fast,
    Smooth,
}

const PREVIEW_SIZE: usize = 256;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Seamless Texture Generator",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )?;

    Ok(())
}
struct MyEguiApp {
    variant: OpenSimplexVariant,
    seed: i32,
    frequency: f32,
    preview: Option<egui::TextureHandle>,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            variant: OpenSimplexVariant::Smooth,
            seed: 1337,
            frequency: 0.01,
            preview: None,
        }
    }
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
        // Generate a preview image of the noise texture based on the current settings one pixel at a time, using the OpenSimplex2 noise function.
    fn generate_preview(&self) -> egui::ColorImage {
        let mut pixels = Vec::with_capacity(PREVIEW_SIZE * PREVIEW_SIZE);
        let radius_x = PREVIEW_SIZE as f64 * self.frequency as f64 / std::f64::consts::TAU;
        let radius_y = radius_x;

        // Generate a seamless texture by sampling the noise function in a circular pattern by iterating over the y and x coordinates of the preview image, calculating the corresponding angles, and using trigonometric functions to determine the sample points in the noise space.
        for y in 0..PREVIEW_SIZE {
            let angle_y = std::f64::consts::TAU * (y as f64 + 0.5) / PREVIEW_SIZE as f64;
            let z = radius_y * angle_y.cos();
            let w = radius_y * angle_y.sin();
            for x in 0..PREVIEW_SIZE {
                let angle_x = std::f64::consts::TAU * (x as f64 + 0.5) / PREVIEW_SIZE as f64;
                let sample_x = radius_x * angle_x.cos();
                let sample_y = radius_x * angle_x.sin();
                let seed = self.seed as i64;
                
                // Call the appropriate OpenSimplex2 noise function based on the selected variant (Fast or Smooth) to generate a noise value for the current sample point in the noise space.
                let value = match self.variant {
                    OpenSimplexVariant::Fast => opensimplex2::fast::noise4_ImproveXY_ImproveZW(
                        seed, sample_x, sample_y, z, w,
                    ),
                    OpenSimplexVariant::Smooth => opensimplex2::smooth::noise4_ImproveXY_ImproveZW(
                        seed, sample_x, sample_y, z, w,
                    ),
                };
                // Convert the noise value to a grayscale color by normalizing it to the range [0, 255] and clamping it to ensure it stays within valid bounds, then push the resulting color to the pixels vector.
                let gray = (((value + 1.0) * 0.5).clamp(0.0, 1.0) * 255.0).round() as u8;
                pixels.push(egui::Color32::from_gray(gray));
            }
        }
        // Create a new ColorImage with the generated pixel data and return it.
        egui::ColorImage::new([PREVIEW_SIZE, PREVIEW_SIZE], pixels)
    }
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Seamless Texture Generator");

            let previous_settings = (self.variant, self.seed, self.frequency);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    egui::ComboBox::from_label("OpenSimplex2 Variant")
                        .selected_text(format!("{:?}", self.variant))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.variant,
                                OpenSimplexVariant::Fast,
                                "OpenSimplex2",
                            );
                            ui.selectable_value(
                                &mut self.variant,
                                OpenSimplexVariant::Smooth,
                                "OpenSimplex2S",
                            );
                        });
                    ui.add(egui::DragValue::new(&mut self.seed).prefix("Seed: "));
                    ui.add(egui::Slider::new(&mut self.frequency, 0.001..=0.1).text("Frequency"));
                });

                ui.separator();

                if let Some(texture) = &self.preview {
                    ui.image((texture.id(), egui::vec2(320.0, 320.0)));
                }
            });

            let current_settings = (self.variant, self.seed, self.frequency);
            if self.preview.is_none() || previous_settings != current_settings {
                let image = self.generate_preview();
                if let Some(texture) = &mut self.preview {
                    texture.set(image, egui::TextureOptions::LINEAR);
                } else {
                    self.preview = Some(ui.ctx().load_texture(
                        "noise-preview",
                        image,
                        egui::TextureOptions::LINEAR,
                    ));
                }
            }
        });
    }
}
