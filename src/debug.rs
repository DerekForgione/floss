#![cfg(debug_assertions)]
#![allow(unused)]

use std::fmt::Debug;

use egui::{self, *};
use crate::ui_extensions::*;
use crate::tasker::*;

// TODO: Bless this mess.
//       It seems that I've discovered a branch of mathematics. I wonder what it's called.
////////////////////////////////////////////////////////////////
/// Single
///     Complete
///         Ballot
///         Bar
/// Inactive
///     Complete
///         
///     Bar   
/// Hovered
/// Focused
/// FocusedAndHovered
/// Pressed
///     Incomplete
/// Multi
///     Complete
///     Incomplete
////////////////////////////////////////////////////////////////
pub struct MockupVisuals {
    fill_color: Color32,
    stroke_color: Color32,
    text_color: Color32,
    /// Optionally modify text size.
    text_size: Option<f32>,
}

impl MockupVisuals {
    /// By default, sets the text_color to the stroke_color.
    pub fn new(fill_color: Color32, stroke_color: Color32) -> Self {
        Self {
            fill_color,
            stroke_color,
            text_color: stroke_color,
            text_size: None,
        }
    }

    pub fn text_color(mut self, value: Color32) -> Self {
        self.text_color = value;
        self
    }

    pub fn text_size(mut self, value: f32) -> Self {
        self.text_size = Some(value);
        self
    }
}

struct MockupStyle {
    
}

pub struct DebugData {
    pub open: bool,
    pub mockup_open: bool,
    pub tasks: TaskList,
}

impl Default for DebugData {
    fn default() -> Self {
        Self { 
            open: false,
            mockup_open: false,
            tasks: TaskList::new(),
        }
    }
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
        ui.allocate_ui(size, |ui| {
            let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
            ui.painter().rect(
                rect,
                Rounding::none(), 
                Color32::from_rgb(0, 18, 51),
                Stroke::new(1.0, Color32::from_rgb(0, 96, 96))
            );
            ui.button("WTF");
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