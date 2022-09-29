#![cfg(debug_assertions)]
#![allow(unused)]

use std::fmt::Debug;

use egui::{self, *};
use crate::ui_extensions::*;
use crate::task::*;

#[macro_export]
macro_rules! print_unimplemented_message {
    () => {
        #[cfg(debug_assertions)]
        println!("Unimplemented on line {}.", line!())
    };
    ($msg:expr) => {
        #[cfg(debug_assertions)]
        println!("Unimplemented on line {}. ({})", line!(), $msg)
    }
}

pub(crate) use print_unimplemented_message;

// TODO: Bless this mess.
//       It seems that I've discovered a branch of mathematics. I wonder what it's called.
////////////////////////////////////////////////////////////////
/// Elements:
///     Single
///     Multi
/// States:
///     Inactive
///     Hovered
///     Focused
///     FocusedAndHovered
///     Pressed
/// Widgets:
///     Ballot (Checkbox like control)
///     Bar (Horizontal bar containing elements related to the Task)
///     
////////////////////////////////////////////////////////////////
/// Inactive
///     Single
///         Bar
///             Complete
///             Incomplete
/// Inactive: status
///     Complete: state
///         Single: type
///             Bar: element
///             Ballot: element
///         Multi: type
///             Bar: element
///             Ballot: element
///     Incomplete: state
///         Single: type
///             Bar: element
///             Ballot: element
///         Multi: type
///             Bar: element
///             Ballot: element
/// Hovered
///     Complete
///         Single
///             Bar
///             Ballot
///         Multi
///             Bar
///             Ballot
///     Incomplete
///         Single
///             Bar
///             Ballot
///         Multi
///             Bar
///             Ballot
/// Focused
/// FocusedAndHovered
/// Pressed
///     Incomplete
/// Multi
///     Complete
///     Incomplete
/// 
/// Inactive
///     Complete
///         Single
///         Bar
///         Ballot
///     Incomplete
/// Hovered
/// Focused
/// FocusedAndHovered
/// Pressed
/// 
/// 
////////////////////////////////////////////////////////////////

pub struct DebugData {
    pub open: bool,
    pub mockup_open: bool,
    pub ballot_state: BallotState,
    pub tasks: TaskList,
}

impl Default for DebugData {
    fn default() -> Self {
        Self { 
            open: false,
            mockup_open: false,
            ballot_state: BallotState::Inactive,
            tasks: TaskList::new(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum BallotState {
    Inactive,
    Active,
    /// Third state that is set when clicked with secondary button.
    TriState,
}

fn ballot(ui: &mut Ui, state: &mut BallotState) -> Response {
    let size = ui.spacing().interact_size.y;
    let (rect, mut response) = ui.allocate_exact_size(vec2(size,size), Sense::click());
    if response.clicked_by(PointerButton::Primary) {
        *state = match *state {
            BallotState::Inactive => BallotState::Active,
            BallotState::Active => BallotState::Inactive,
            /// Tristate can only be changed via right click.
            BallotState::TriState => BallotState::TriState,
        };
        response.mark_changed();
    } else if response.clicked_by(PointerButton::Secondary) {
        *state = match *state {
            /// Tristate can only be changed via right click.
            BallotState::TriState => BallotState::Inactive,
            BallotState::Inactive => BallotState::TriState,
            _ => BallotState::TriState,
        };
        response.mark_changed();
    }
    if ui.is_rect_visible(rect) {
        
        let visuals = ui.style().interact(&response);

        let painter = ui.painter();
        painter.rect(
            rect.shrink(3.0),
            Rounding::none(),
            // Color32::from_rgb(36, 0, 70),
            visuals.bg_fill,
            visuals.bg_stroke,
        );
        let color = match *state {
            BallotState::Inactive => None,
            BallotState::Active => Some(Color32::from_rgb(255, 195, 0)),
            BallotState::TriState => Some(Color32::RED),
        };
        if let Some(color) = color {
            let dot_rect = rect.shrink(5.0);
            painter.rect(
                dot_rect,
                Rounding::none(),
                color,
                visuals.bg_stroke,
            );  
        }
        
    }
    
    response

}


impl DebugData {
    pub fn new() -> Self {
        Default::default()
    }

    fn window_ui(&mut self, ui: &mut Ui) {
        // For the first test, I will Draw the mockup UI
        // 24x24, 16x16, 10x10
        let width = ui.available_width();
        let size = Vec2::new(width, 24.0);
        egui::Window::new("Do you want to quit?")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        println!("Cancel");
                    }

                    if ui.button("Yes").clicked() {
                        println!("Yes");
                    }
      
                });
            });
        ui.allocate_ui(size, |ui| {
            let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
            ui.painter().rect(
                rect,
                Rounding::none(), 
                Color32::from_rgb(0, 18, 51),
                Stroke::new(1.0, Color32::from_rgb(0, 96, 96))
            );
            ui.button("WTF");
            let mut value: &str = "";
            FunctionBar::new()
                .action("test_btn1", || {
                    println!("test_btn1 was clicked.");
                }).id(Id::new("test_btn1"))
                .action("test_btn2", || {
                    println!("test_btn2 was clicked.");
                }).id(Id::new("test_btn2"))
                .action("test_btn3", || {
                    println!("test_btn3 was clicked.");
                }).id(Id::new("test_btn3"))
                .show(ui);
        });
        let mut checked = self.ballot_state == BallotState::Active;
        if ui.ballot(&mut checked).changed() {
            self.ballot_state = if checked { BallotState::Active } else { BallotState::Inactive };
        }
        function_bar![ui;
            "Test" => {
                println!("This is a test.");
            }
            "Another Button" => {
                println!("The quick brown fox jumps over the lazy dog.");
            }
            RichText::new("Red Button").color(Color32::RED) => {
                println!("The red button was clicked.");
            }
        ];

        Frame::popup(ui.style()).show(ui, |ui| {
            if ui.button("This is a popup.").clicked() {
                println!("Just gotta do it.");
            }
        });

        ui.menu_button("Test", |ui| {
            ui.label(RichText::new("Test").color(Color32::RED));
        });
        
    }

    pub fn show(&mut self, ctx: &Context) {
        let mut open = self.open;
        Window::new(
            RichText::new("[Debug]")
                .color(Color32::RED)
                .size(24.0)
            )
            .open(&mut open)
            .show(ctx, |ui| self.window_ui(ui));
        self.open = open;
    }

}