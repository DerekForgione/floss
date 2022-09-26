use crate::tasker::{self, *};
use egui::{self, *};

struct NewTaskPopup {
    // title, description
    pub title: String,
    pub description: Option<String>,
}

impl Default for NewTaskPopup {
    fn default() -> Self {
        Self {
            title: "".to_owned(),
            description: None,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]// if we add new fields, give them default values when deserializing old state
pub struct FlossApp {
    tasks: Vec<Task>,
    #[serde(skip)]
    new_popup: NewTaskPopup,
}

impl Default for FlossApp {
    fn default() -> Self {
        Self {
           tasks: Vec::new(),
           new_popup: NewTaskPopup { title: "".to_owned(), description: None }
        }
    }
}

impl FlossApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

const POPUP_WIDTH: f32 = 300.0;

impl eframe::App for FlossApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { tasks, new_popup } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Add Task", |ui| {
                    ui.group(|ui| {
                        ui.allocate_ui(Vec2::new(POPUP_WIDTH, ui.spacing().interact_size.y), |ui| {
                            ui.put(ui.max_rect(), text_edit::TextEdit::singleline(&mut new_popup.title).hint_text("Title"));
                        });
                        let mut clear_desc = false;
                        if let Some(description) = &mut new_popup.description {
                            ui.allocate_ui(Vec2::new(POPUP_WIDTH, ui.spacing().interact_size.y * 4.0), |ui| {
                                let empty_before = description.is_empty();
                                let resp = ui.put(ui.max_rect(), text_edit::TextEdit::multiline(description).hint_text("Description (Press [Delete] To Remove)"));
                                if resp.has_focus() && description.is_empty() && empty_before {
                                    if ui.input().key_pressed(Key::Delete) {
                                        clear_desc = true;
                                    }
                                }
                            });
                            if new_popup.title.is_empty() {
                                ui.allocate_ui(Vec2::new(POPUP_WIDTH, ui.spacing().interact_size.y), |ui| {
                                    let btn = ui.put(ui.max_rect(), Button::new("Remove Description"));
                                    if btn.clicked() {
                                        new_popup.description = None;
                                    }
                                });
                            }
                        } else {
                            if new_popup.title.is_empty() {
                                ui.allocate_ui(Vec2::new(POPUP_WIDTH, ui.spacing().interact_size.y), |ui| {
                                    let btn = ui.put(ui.max_rect(), Button::new("Add Description"));
                                    if btn.clicked() {
                                        new_popup.description = Some("".to_owned());
                                    }
                                });
                            }
                        }
                        if clear_desc {
                            new_popup.description = None;
                        }
                        if !new_popup.title.is_empty() {
                            ui.allocate_ui(Vec2::new(POPUP_WIDTH, ui.spacing().interact_size.y), |ui| {
                                ui.columns(2, |cols| {
                                    cols[0].columns(2, |cols| {
                                        let create = cols[0].put(cols[0].max_rect(), Button::new("Add"));
                                        let clear = cols[1].put(cols[1].max_rect(), Button::new("Clear"));
                                        if create.clicked() {
                                            tasks.push(Task::new(new_popup.title.clone(), cols[0].next_id()));
                                            new_popup.title = "".to_owned();
                                            new_popup.description = None;
                                        }
                                        if clear.clicked() {
                                            new_popup.title = "".to_owned();
                                            new_popup.description = None;
                                        }
                                    });
                                    let desc_title = if new_popup.description.is_some() {
                                        "Remove Description".to_owned()
                                    } else {
                                        "Add Description".to_owned()
                                    };
                                    let desc_btn = cols[1].put(cols[1].max_rect(), Button::new(desc_title));
                                    if desc_btn.clicked() {
                                        if new_popup.description.is_some() {
                                            new_popup.description = None;
                                        } else {
                                            new_popup.description = Some("".to_owned());
                                        }
                                    }
                                });
                            });
                        }

                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                tasker::render_list(ui, tasks, AdderPosition::Above);
            });
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
