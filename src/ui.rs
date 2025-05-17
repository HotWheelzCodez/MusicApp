use eframe::egui;
use egui::{Color32, CornerRadius};

struct ButtonStyle {
    base_color: Color32,
    hover_color: Color32,
    text_color: Color32,
    text_hover_color: Color32,
    outline_color: Color32,
    outline_hover_color: Color32,
    radius: u8,
    outline_width: f32,
}

fn button(ui: &mut egui::Ui, button_style: &ButtonStyle, text: &str) -> egui::Response {
    let desired_size = egui::Vec2::new(80.0, 30.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    let mut response = response;
    let (bg_color, text_color, outline_color) = if response.hovered() {
        response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        (button_style.hover_color, button_style.text_hover_color, button_style.outline_hover_color)
    } else {
        (button_style.base_color, button_style.text_color, button_style.outline_color)
    };

    ui.painter().rect(
        rect,
        CornerRadius::same(button_style.radius),
        bg_color,
        egui::Stroke::new(button_style.outline_width, outline_color),
        egui::StrokeKind::Inside
    );

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(14.0),
        text_color,
    );

    response
}

#[derive(Default)]
struct MyEguiApp {

}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Play set", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}
