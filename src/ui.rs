use eframe::egui;
use egui::{Color32, CornerRadius};
use crate::playset;
use crate::music_player;
use crate::playset::Library;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;

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
    let desired_size = egui::Vec2::new(100.0, 30.0);
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

struct MyEguiApp {
    display_menu: bool,
    library_name: String,
    stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sink: Sink,
    library: playset::Library,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        Self {
            display_menu: false,
            stream: Some(stream),
            sink: Sink::try_new(&handle).unwrap(),
            stream_handle: Some(handle),
            library: Library::initialize("./song_library/U", "./song_library/subsets").unwrap(),
            library_name: "".to_string()
        }
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
                        // let song = "song_library/U/Vylet Pony - CUTIEMARKS (And the Things That Bind Us) - 14 HOW TO KILL A MONSTER.mp3";
                        // music_player::play_music(song, &self.stream_handle, &self.sink);
                        music_player::get_youtube_music("https://www.youtube.com/watch?v=T2nBvNBzrP8");

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
