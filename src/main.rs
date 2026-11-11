use fastnoise_lite::{FastNoiseLite, NoiseType};
use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));

    let mut noise = FastNoiseLite::with_seed(80085);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));

    let value = noise.get_noise_2d(12.0, 34.0);
    println!("Noise value: {value}");
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ui, |ui| {
           ui.heading("Hello World!");
       });
   }
}