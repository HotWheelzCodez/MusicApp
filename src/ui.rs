use eframe::egui;
use egui::TextBuffer;
use egui::{Color32, CornerRadius};
use crate::playset;
use crate::music_player;
use crate::playset::Library;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::collections::HashSet;

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

const GLOBAL_BUTTON_STYLE: ButtonStyle = ButtonStyle {
    base_color: Color32::from_rgb(200, 50, 180),
    hover_color: Color32::from_rgb(180, 30, 160),
    text_color: Color32::WHITE,
    text_hover_color: Color32::WHITE,
    outline_color: Color32::WHITE,
    outline_hover_color: Color32::WHITE,
    radius: 3,
    outline_width: 1.0,
};

fn button(ui: &mut egui::Ui, button_style: &ButtonStyle, text: &str, dims: egui::Vec2) -> egui::Response {
    let desired_size = dims;
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
    songs_to_show: HashSet<playset::Song>,
    show_songs: bool,
    playing: String,
    display_set_menu: bool,
    selected_set: playset::Playset,
    selected_transformation: String,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        let library = Library::initialize("./song_library/U", "./song_library/subsets").unwrap();
        let set = library.universal_set.clone();
        Self {
            display_menu: false,
            stream: Some(stream),
            sink: Sink::try_new(&handle).unwrap(),
            stream_handle: Some(handle),
            library,
            selected_set: set,
            library_name: "".to_string(),
            songs_to_show: HashSet::new(),
            show_songs: false,
            playing: String::new(),
            display_set_menu: false,
            selected_transformation: String::from("Union"),
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Play set").color(Color32::from_rgb(200, 50, 180)).size(50.0));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.show_songs {
                    } else {
                        if button(ui, &GLOBAL_BUTTON_STYLE, "Create Play set!", egui::Vec2::new(100.0, 30.0)).clicked() {
                            self.display_menu = true;
                        }
                    }
                });
            });

            if self.display_menu {
                egui::Window::new("Add").resizable([false, false]).default_size((400.0, 400.0)).show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Play set name");
                        let response = ui.add(egui::TextEdit::singleline(&mut self.library_name));
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) || button(ui, &GLOBAL_BUTTON_STYLE, "Add", egui::Vec2::new(50.0, 30.0)).clicked() {
                            Library::push_empty_set(&mut self.library, self.library_name.clone()); 
                            self.library_name.clear();
                            self.display_menu = false;
                        }
                    })
                });
            }

            if self.display_set_menu {
                let transformations = vec!["Union", "Difference", "Intersection"];
                egui::Window::new("Setup your sets").resizable([false, false]).default_size((400.0, 400.0)).show(ctx, |ui| {
                    egui::ComboBox::from_label("Select Set").selected_text(format!("{}", self.selected_set.name)).show_ui(ui, |ui| {
                        for option in &self.library.sets {
                            let (name, _) = option.clone();
                            let clone = name.clone();
                            ui.selectable_value(&mut self.selected_set.name, clone.clone(), clone);
                        }
                    });
                    egui::ComboBox::from_label("Select Transformation").selected_text(format!("{}", self.selected_transformation)).show_ui(ui, |ui| {
                        for option in transformations {
                            ui.selectable_value(&mut self.selected_transformation, option.to_string(), option);
                        }
                    });
                    if button(ui, &GLOBAL_BUTTON_STYLE, "Transform Set", egui::Vec2::new(100.0, 15.0)).clicked() {
                        self.selected_transformation = String::from("Union");
                        self.selected_set = self.library.universal_set.clone();
                        self.display_set_menu = false;
                    }
                });
            }
        });

        if self.show_songs {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if button(ui, &GLOBAL_BUTTON_STYLE, "Back to Play set's", egui::Vec2::new(110.0, 30.0)).clicked() {
                        self.show_songs = false;
                        self.songs_to_show.clear();
                        return;
                    }
                    if button(ui, &GLOBAL_BUTTON_STYLE, "Add Set Connection", egui::Vec2::new(120.0, 30.0)).clicked() {
                        self.display_set_menu = true; 
                    }
                    // if button(ui, &GLOBAL_BUTTON_STYLE, "Add song", egui::Vec2::new(120.0, 30.0)).clicked() {
                    // }
                });
                ui.vertical(|ui| {
                    for song in &self.songs_to_show {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.label(&song.name);
                                ui.label(&song.album);
                                ui.label(&song.artist);
                                //ui.label(song.duration.to_string());
                            });
                            ui.separator();
 
                            let mut text = "Pause";
                            if self.sink.is_paused() || self.sink.empty() {
                                text = "Play";
                            }
                            if button(ui, &GLOBAL_BUTTON_STYLE, text, egui::Vec2::new(75.0, 30.0)).clicked() {
                                if self.sink.empty() {
                                    music_player::play_music(&format!("song_library/U/{}", song.name), &self.stream_handle, &self.sink);
                                    self.sink.play();
                                    let (left, _) = song.name.rsplit_once('.').unwrap();
                                    self.playing = left.to_string();
                                } else if self.sink.is_paused() {
                                    self.sink.play();
                                } else {
                                    self.sink.pause();
                                }
                            }
                        });
                    }
                });
            });

            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new("Music").color(Color32::from_rgb(200, 50, 180)).size(20.0));
            ui.add_space(10.0);

            let items_per_row = 4;

            let sets = &self.library.sets;
            let sets_len = sets.len();

            let vecified: Vec<(&String, &playset::Playset)> = sets.iter().collect();

            let mut i = 0;
            while i < sets_len {
                ui.horizontal(|ui| {
                    for j in 0..items_per_row {
                        let index = i + j;
                        if index >= sets_len {
                            break;
                        }

                        let (name, playset) = vecified[index];
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.label(name);
                            });
                            ui.separator();

                            if button(ui, &GLOBAL_BUTTON_STYLE, "Open", egui::Vec2::new(50.0, 30.0)).clicked() {
                                self.songs_to_show = playset.songs.flatten(&self.library.sets);
                                self.show_songs = true;
                            }
                        });
                    }
                });
                i += items_per_row;
            }
        });

        egui::TopBottomPanel::bottom("player").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.playing);

                let text = if self.sink.is_paused() { "Play" } else { "Pause" };
                if button(ui, &GLOBAL_BUTTON_STYLE, text, egui::Vec2::new(100.0, 30.0)).clicked() {
                    if self.sink.is_paused() {
                        self.sink.play();
                    } else {
                        self.sink.pause();
                    }
                }
            });
        });
   }
}

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Play set", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}
