use eframe::egui;
use egui::{Color32, CornerRadius};
use crate::playset::{self, playset};

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
    display_menu: bool,
    library_name: String,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Play set").color(Color32::from_rgb(200, 50, 180)).size(50.0));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if button(ui, &ButtonStyle {
                        base_color: Color32::from_rgb(200, 50, 180),
                        hover_color: Color32::from_rgb(180, 30, 160), text_color: Color32::WHITE,
                        text_hover_color: Color32::WHITE,
                        outline_color: Color32::WHITE,
                        outline_hover_color: Color32::WHITE,
                        radius: 2,
                        outline_width: 1.0
                    }, "Create Play set!").clicked() {
                        self.display_menu = true;
                    }
                });
            });

            if self.display_menu {
                egui::Window::new("Add Music").resizable([false, false]).default_size((400.0, 400.0)).show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Play set name");
                        let response = ui.add(egui::TextEdit::singleline(&mut self.library_name));
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            
                        }
                    })
                });
            }
        });
   }
}

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Play set", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}
