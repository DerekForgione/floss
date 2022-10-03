use crate::task::{self, *};
use eframe::epaint::Shadow;
use egui::{self, *, style::Margin};
use crate::ui_extensions::*;
use crate::debug::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]// if we add new fields, give them default values when deserializing old state
pub struct FlossApp {
    tasks: Vec<Task>,
    #[serde(skip)]
    new_popup: NewTaskPopupButton,
    // Debug Stuff
    #[cfg(debug_assertions)]
    #[serde(skip)]
    sample_list: Vec<Task>,
    #[cfg(debug_assertions)]
    #[serde(skip)]
    testwin_open: bool,
    #[cfg(debug_assertions)]
    #[serde(skip)]
    debug: DebugData,
}

impl Default for FlossApp {
    fn default() -> Self {
        Self {
           tasks: TaskList::new(),
           new_popup: Default::default(),
           #[cfg(debug_assertions)]
           sample_list: TaskList::new(),
           #[cfg(debug_assertions)]
           testwin_open: false,
           #[cfg(debug_assertions)]
           debug:DebugData::new(),
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
        let Self { 
            tasks, 
            new_popup, 
            #[cfg(debug_assertions)]
            sample_list, 
            #[cfg(debug_assertions)]
            testwin_open,
            #[cfg(debug_assertions)]
            debug,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.style_mut().visuals.window_rounding = Rounding::none();
            ui.style_mut().visuals.popup_shadow = Shadow::default();
            ui.style_mut().visuals.widgets.active.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.inactive.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.hovered.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.open.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.noninteractive.rounding = Rounding::none();
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    #[cfg(debug_assertions)]
                    if ui.button("Debug").clicked() {
                        *testwin_open = true;
                        debug.open = true;
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                new_popup.show(ui, "⊞", tasks);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals.window_rounding = Rounding::none();
            ui.style_mut().visuals.popup_shadow = Shadow::default();
            ui.style_mut().visuals.widgets.active.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.inactive.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.hovered.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.open.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.noninteractive.rounding = Rounding::none();
            ScrollArea::vertical().show(ui, |ui| {
                task::render_list(ui, tasks);
            });
            egui::warn_if_debug_build(ui);
        });

        #[cfg(debug_assertions)]
        if true {
            debug.show(ctx);
            egui::Window::new("Test GUI").open(testwin_open).show(ctx, |ui| {
                ui.style_mut().visuals.window_rounding = Rounding::none();
                ui.style_mut().visuals.popup_shadow = Shadow::default();
                ui.style_mut().visuals.widgets.active.rounding = Rounding::none();
                ui.style_mut().visuals.widgets.inactive.rounding = Rounding::none();
                ui.style_mut().visuals.widgets.hovered.rounding = Rounding::none();
                ui.style_mut().visuals.widgets.open.rounding = Rounding::none();
                ui.style_mut().visuals.widgets.noninteractive.rounding = Rounding::none();
                // Sample Task UIm
                ui.horizontal(|top| {
                    if sample_list.is_empty() {
                        let create = top.put(top.max_rect(), Button::new("Create New Task."));
                        if create.clicked() {
                            sample_list.push(Task::new("Untitled Task"));
                        }
                        create
                    } else {
                        let clear = top.put(top.max_rect(), Button::new("Remove All Tasks"));
                        if clear.clicked() {
                            sample_list.clear();
                        }
                        clear
                    }
                });
                if !sample_list.is_empty() {
                    ui.allocate_ui(Vec2::new(400.0, 500.0), |ui| { ui.vertical(|ui| {
                        Frame::group(ui.style())
                        .outer_margin(Margin::same(0.0))
                        .inner_margin(Margin::same(0.0))
                        .rounding(Rounding::none())
                        .show(ui, |ui| {
                            for task in sample_list.iter_mut() {
                                ui.horizontal(|ui| {
                                    let mut check = task.complete();
                                    if ui.ballot(&mut check).changed() {
                                        task.toggle_complete();
                                    }
                                    ui.style_mut().override_text_style = Some(TextStyle::Heading);
                                    ui.label(&task.title);
                                });
                                
                            }
                        });
                    });}); // win.allocate_ui
                    if ui.button("Another One").clicked() {
                        sample_list.push(Task::new("Untitled"));
                    }
                }
                let result = ui.button_bar(&[
                    ("Hello", "The quick brown fox jumps over the lazy dog."),
                    ("Test1", "This is a test."),
                    ("Test2", "This is a test."),
                    ("Test3", "This is a test."),
                    ("Test4", "This is a test."),
                ]);
                if result.response.changed() {
                    println!("{}", result.inner.unwrap_or("<NONE>"));
                }
                ui.icon(Icon::Info);
                ui.horizontal(|ui| {
                    if ui.icon_button(Icon::Gear).clicked() {
                        println!("Interact Height: {}", ui.spacing().interact_size.y);
                        println!("Available Height: {}", ui.available_height());
                    }
                });
                ui.with_layout(Layout::top_down(Align::Min).with_cross_justify(true), |ui| {
                    ui.style_mut().override_text_style = Some(TextStyle::Monospace);
                    Frame::group(ui.style()).inner_margin(Margin::same(0.0)).show(ui, |ui| {
                        _ = ui.button("Test");
                        _ = ui.button("Another button");
                    });
                });
            });
        }
    }
}
