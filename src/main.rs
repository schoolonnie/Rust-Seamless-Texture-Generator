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
    fractal_enabled: bool,
    octaves: i32,
    lacunarity: f32,
    gain: f32,
    warp_enabled: bool,
    warp_strength: f32,
    warp_frequency: f32,
    preview: Option<egui::TextureHandle>,
    tiled_preview: Option<egui::TextureHandle>,
    tiling_factor: usize,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            variant: OpenSimplexVariant::Smooth,
            seed: 1337,
            frequency: 0.01,
            fractal_enabled: false,
            octaves: 4,
            lacunarity: 2.0,
            gain: 0.5,
            warp_enabled: false,
            warp_strength: 0.5,
            warp_frequency: 2.0,
            preview: None,
            tiled_preview: None,
            tiling_factor: 4, // Default tiling factor
        }
    }
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }

    fn sample_noise(&self, seed: i64, point: [f64; 4]) -> f32 {
        match self.variant {
            OpenSimplexVariant::Fast => opensimplex2::fast::noise4_ImproveXY_ImproveZW(
                seed, point[0], point[1], point[2], point[3],
            ),
            OpenSimplexVariant::Smooth => opensimplex2::smooth::noise4_ImproveXY_ImproveZW(
                seed, point[0], point[1], point[2], point[3],
            ),
        }
    }

    // Generate a preview image of the noise texture based on the current settings one pixel at a time, using the OpenSimplex2 noise function
    fn generate_preview(&self) -> egui::ColorImage {
        let mut pixels = Vec::with_capacity(PREVIEW_SIZE * PREVIEW_SIZE);
        let radius_x = PREVIEW_SIZE as f64 * self.frequency as f64 / std::f64::consts::TAU;
        let radius_y = radius_x;

        // Generate a seamless texture by sampling the noise function in a circular pattern by iterating over the y and x coordinates of the preview image, calculating the corresponding angles, and using trigonometric functions to determine the sample points in the noise space
        for y in 0..PREVIEW_SIZE {
            let angle_y = std::f64::consts::TAU * (y as f64 + 0.5) / PREVIEW_SIZE as f64;
            let z = radius_y * angle_y.cos();
            let w = radius_y * angle_y.sin();
            for x in 0..PREVIEW_SIZE {
                let angle_x = std::f64::consts::TAU * (x as f64 + 0.5) / PREVIEW_SIZE as f64;
                let seed = self.seed as i64;
                let base_point = [radius_x * angle_x.cos(), radius_x * angle_x.sin(), z, w];
                // If domain warping is enabled, apply the warp to the sample point by calculating offsets based on the noise function and the warp frequency and strength, otherwise use the base point directly
                let sample_point = if self.warp_enabled {
                    let warp_point =
                        base_point.map(|coordinate| coordinate * self.warp_frequency as f64);
                    let offsets = [
                        self.sample_noise(seed.wrapping_add(1), warp_point),
                        self.sample_noise(seed.wrapping_add(2), warp_point),
                        self.sample_noise(seed.wrapping_add(3), warp_point),
                        self.sample_noise(seed.wrapping_add(4), warp_point),
                    ];
                    // Apply the offsets to the base point, scaling them by the warp strength to create a warped sample point
                    std::array::from_fn(|axis| {
                        base_point[axis] + offsets[axis] as f64 * self.warp_strength as f64
                    })
                } else {
                    // else, use base point (domain warp disabled)
                    base_point
                };
                // Determine the number of octaves to use for the noise generation based on whether fractal noise is enabled, and initialize variables for amplitude, frequency, and sums for value and amplitude
                let octave_count = if self.fractal_enabled {
                    self.octaves
                } else {
                    1
                };
                let mut amplitude = 1.0_f32;
                let mut frequency = 1.0_f64;
                let mut value_sum = 0.0_f32;
                let mut amplitude_sum = 0.0_f32;

                // Iterate through each octave, scaling the sample point and accumulating the noise values and amplitudes to create a fractal noise effect
                for _ in 0..octave_count {
                    // Scaling all four coordinates keeps each octave periodic.
                    let octave_point = sample_point.map(|coordinate| coordinate * frequency);
                    let octave_value = self.sample_noise(seed, octave_point);

                    value_sum += octave_value * amplitude;
                    amplitude_sum += amplitude;
                    amplitude *= self.gain;
                    frequency *= if self.fractal_enabled {
                        self.lacunarity as f64
                    } else {
                        1.0
                    };
                }

                let value = value_sum / amplitude_sum;
                // Convert the noise value to a grayscale color by normalizing it to the range [0, 255] and clamping it to ensure it stays within valid bounds, then push the resulting color to the pixels vector
                let gray = (((value + 1.0) * 0.5).clamp(0.0, 1.0) * 255.0).round() as u8;
                pixels.push(egui::Color32::from_gray(gray));
            }
        }
        // Create a new ColorImage with the generated pixel data and return it
        egui::ColorImage::new([PREVIEW_SIZE, PREVIEW_SIZE], pixels)
    }

    fn tile_preview(image: &egui::ColorImage, tiling_factor: usize) -> egui::ColorImage {
        let [width, height] = image.size;
        let mut pixels = Vec::with_capacity(width * tiling_factor * height * tiling_factor);

        for y in 0..height * tiling_factor {
            for x in 0..width * tiling_factor {
                pixels.push(image.pixels[(y % height) * width + (x % width)]);
            }
        }

        egui::ColorImage::new([width * tiling_factor, height * tiling_factor], pixels)
    }
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Seamless Texture Generator");

            // Store the previous settings to check for changes later
            let previous_settings = (
                self.variant,
                self.seed,
                self.frequency,
                self.fractal_enabled,
                self.octaves,
                self.lacunarity,
                self.gain,
                self.warp_enabled,
                self.warp_strength,
                self.warp_frequency,
                self.tiling_factor,
            );

            // Create a horizontal layout for the controls section
            ui.horizontal(|ui| {
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
                ui.checkbox(&mut self.fractal_enabled, "Enable fBm");
                if self.fractal_enabled {
                    ui.add(egui::Slider::new(&mut self.octaves, 1..=8).text("Octaves"));
                    ui.add(egui::Slider::new(&mut self.lacunarity, 1.0..=4.0).text("Lacunarity"));
                    ui.add(egui::Slider::new(&mut self.gain, 0.0..=1.0).text("Gain"));
                }
                ui.checkbox(&mut self.warp_enabled, "Enable domain warp");
                if self.warp_enabled {
                    ui.add(
                        egui::Slider::new(&mut self.warp_strength, 0.0..=2.0).text("Warp strength"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.warp_frequency, 0.5..=8.0)
                            .text("Warp frequency"),
                    );
                }
                ui.add(egui::Slider::new(&mut self.tiling_factor, 2..=8).text("Tiling Factor"));
            });

            // Add a separator between the controls and the preview images and create a two-column layout for the preview images
            ui.separator();
            ui.columns(2, |columns| {
                columns[0].label("Single Tile");
                columns[1].label(format!(
                    "{} x {} Tiling",
                    self.tiling_factor, self.tiling_factor
                ));

                if let Some(texture) = &self.preview {
                    columns[0].image((texture.id(), egui::vec2(320.0, 320.0)));
                }
                if let Some(texture) = &self.tiled_preview {
                    columns[1].image((texture.id(), egui::vec2(320.0, 320.0)));
                }
            });

            // Check if the settings have changed or if the preview is not generated yet, and generate a new preview image if necessary
            let current_settings = (
                self.variant,
                self.seed,
                self.frequency,
                self.fractal_enabled,
                self.octaves,
                self.lacunarity,
                self.gain,
                self.warp_enabled,
                self.warp_strength,
                self.warp_frequency,
                self.tiling_factor,
            );
            if self.preview.is_none() || previous_settings != current_settings {
                let image = self.generate_preview();
                let tiled_image = Self::tile_preview(&image, self.tiling_factor);
                if let Some(texture) = &mut self.preview {
                    texture.set(image, egui::TextureOptions::LINEAR);
                } else {
                    self.preview = Some(ui.ctx().load_texture(
                        "noise-preview",
                        image,
                        egui::TextureOptions::LINEAR,
                    ));
                }
                if let Some(texture) = &mut self.tiled_preview {
                    texture.set(tiled_image, egui::TextureOptions::LINEAR);
                } else {
                    self.tiled_preview = Some(ui.ctx().load_texture(
                        "noise-preview-tiled",
                        tiled_image,
                        egui::TextureOptions::LINEAR,
                    ));
                }
            }
        });
    }
}
