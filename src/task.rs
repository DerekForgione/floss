#![allow(unused)]

use serde::*;

use std::{time::Instant, collections::HashMap};
use chrono::prelude::*;

use eframe::epaint::Shadow;
use egui::{self, *, text_edit::TextEdit};

use crate::{ui_extensions::UiExtensions, print_unimplemented_message, function_bar};

/// TODO
///  Create clocked task. The clocked task will have a start/stop button which will measure the
///     amount of time spent on a task.  
//////


pub type TaskList = Vec<Task>;

pub trait AtomicId {
    fn next_id(&self) -> Id;
    fn next_index(&self) -> u64;
}

fn global_index() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

impl AtomicId for Ui {

    fn next_id(&self) -> Id {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        self.id().with(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    fn next_index(&self) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

}

fn next_index() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum TaskData {
    Single(bool),
    Multiple(Vec<Task>),
}

impl TaskData {

    pub fn multi() -> Self {
        Self::Multiple(Vec::new())
    }

    pub fn single() -> Self {
        Self::Single(false)
    }

    pub fn tasks(&mut self) -> &mut Vec<Task> {
        if let TaskData::Multiple(tasks) = self {
            return tasks;
        } else {
            panic!("Wtf dude.");
        }
    }

    pub fn task_count(&self) -> usize {
        match self {
            TaskData::Single(_) => 1,
            TaskData::Multiple(tasks) => {
                let mut count = 0usize;
                for task in tasks.iter() {
                    count += task.data.task_count();
                }
                count
            }
        }
    }

    pub fn completed_count(&self) -> usize {
        match self {
            TaskData::Single(done) => if *done { 1 } else { 0 },
            TaskData::Multiple(tasks) => {
                let mut count = 0usize;
                for task in tasks.iter() {
                    count += task.data.completed_count();
                }
                count
            }
        }
    }

    pub fn completion_percent(&self) -> f64 {
        let total = self.task_count();
        if total == 0 {
            0.0
        } else {
            ((self.completed_count() / total) as f64) * 100.0
        }
    }

}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub id: Id,
    pub creation_time: chrono::DateTime<Utc>,
    pub data: TaskData,
    #[serde(skip)]
    remove_me: bool,
    #[serde(skip)]
    editing: bool,
}

impl Task {
    pub fn new<T: Into<String>>(title: T) -> Self {
        let time = Utc::now();
        Self {
            title: title.into(),
            description: None,
            id: Id::new(time),
            creation_time: time,
            data: TaskData::single(),
            remove_me: false,
            editing: false,
        }
    }

    pub fn group<T: Into<String>>(title: T) -> Self {
        Self::new(title)
            .with(TaskList::new())
    }

    pub fn with(mut self, tasks: TaskList) -> Self {
        self.data = TaskData::Multiple(tasks);
        self
    }

    pub fn set_title<T: Into<String>>(&mut self, text: T) {
        self.title = text.into();
    }

    pub fn title<T: Into<String>>(mut self, text: T) -> Self {
        self.set_title(text);
        self
    }

    pub fn set_description<T: Into<String>>(&mut self, text: T) {
        self.description = Some(text.into());
    }

    pub fn description<T: Into<String>>(mut self, text: T) -> Self {
        self.description = Some(text.into());
        self
    }

    pub fn single(&self) -> bool {
        match self.data {
            TaskData::Single(_) => true,
            _ => false,
        }
    }

    pub fn multi(&self) -> bool {
        match self.data {
            TaskData::Multiple(_) => true,
            _ => false,
        }
    }

    pub fn add(&mut self, task: Task) {
        if let TaskData::Multiple(items) = &mut self.data {
            items.push(task);
        } else {
            self.data = TaskData::Multiple(vec![task]);
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Task> {
        if let TaskData::Multiple(items) = &mut self.data {
            Some(items.remove(index))
        } else {
            None
        }
    }

    /// Toggles task completion state and returns the value.
    pub fn toggle_complete(&mut self) -> bool {
        if self.incomplete() {
            self.mark_complete();
            true
        } else {
            self.mark_incomplete();
            false
        }
    }

    pub fn complete(&self) -> bool {
        match &self.data {
            TaskData::Single(done) => *done,
            TaskData::Multiple(tasks) => {
                if tasks.is_empty() {
                    return true;
                }
                for task in tasks.iter() {
                    if task.incomplete() {
                        return false;
                    }
                }
                true
            }
        }
    }

    pub fn mark_complete(&mut self) {
        match &mut self.data {
            TaskData::Single(done) => *done = true,
            TaskData::Multiple(tasks) => {
                tasks.iter_mut().for_each(Task::mark_complete);
            }
        }
    }

    pub fn incomplete(&self) -> bool {
        match &self.data {
            TaskData::Single(done) => !*done,
            TaskData::Multiple(tasks) => {
                if tasks.is_empty() {
                    return false;
                }
                for task in tasks.iter() {
                    if task.incomplete() {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn mark_incomplete(&mut self) {
        match &mut self.data {
            TaskData::Single(done) => *done = false,
            TaskData::Multiple(tasks) => {
                tasks.iter_mut().for_each(Task::mark_incomplete);
            }
        }
    }

    pub fn render_header(&mut self, ui: &mut Ui) -> Response {
        Frame::group(ui.style())
            .rounding(Rounding::none())
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mark =  ui.horizontal (|ui|  {
                        let mut check = self.complete();
                        if self.editing {
                            ui.set_visible(false);
                        }
                        let mark = ui.ballot(&mut check);
                        if mark.changed() {
                            self.toggle_complete();
                        }
                        //let add = ui.small_button("[+]"); //⊞
                        let add = ui.add(
                            Button::new("[+]")
                                .frame(false)
                                // .small()
                                .stroke(Stroke::none())
                        );
                        if add.clicked() {
                            self.add(Task::new("Untitled Task"));
                        }
                        mark
                    }).inner;
                    let title = if self.editing {
                        let resp = ui.text_edit_singleline(&mut self.title);
                        if resp.lost_focus() {
                            self.editing = false;
                        }
                        resp
                    } else {
                        let resp = ui.label(self.title.as_str()).interact(Sense::click());
                        if resp.clicked() {
                            self.editing = true;
                        }
                        resp
                    };
                    let right = ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        let delete = ui.button(RichText::new("⊗").color(Color32::RED));
                        if delete.clicked() {
                            self.remove_me = true;
                        }
                        let edit = ui.button(if self.editing { "💾" } else { "✏" });
                        if edit.clicked() {
                            self.editing = !self.editing;
                        }
                        delete | edit
                    }).inner;
                    mark | title | right
                }).response
            }).inner
    }

    pub fn render(&mut self, ui: &mut Ui) -> Response {
        // This task will either be a single or a multi
        // TODO: Capture all Responses from ui to return to caller.
        let id = ui.make_persistent_id(self.creation_time);
        if self.single() {
            ui.push_id(id, |ui| self.render_header(ui)).inner
        } else {
            collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
                .show_header(ui, |ui| ui.horizontal(|ui| {
                    self.render_header(ui)
                }))
                .body(|ui| {
                    let tasks = self.data.tasks();
                    render_list(ui, tasks);
                }).0
        }
        
    }
    
}

pub fn render_list(ui: &mut Ui, tasks: &mut Vec<Task>) -> Response {
    ui.vertical(|ui| {

        if !tasks.is_empty() {
            // Order by incomplete tasks first.
            let complete: Vec<usize> = tasks.iter()
                .enumerate()
                .filter(|(_, task)| task.incomplete() )
                .map(|(i, _)| i)
                .collect();
            let incomplete: Vec<usize> = tasks.iter()
                .enumerate()
                .filter(|(_, task)| task.complete() )
                .map(|(i, _)| i)
                .collect();
            let closure = |task: &mut Task| { task.render(ui); };
            complete
                .into_iter()
                .for_each(|i| { tasks[i].render(ui); });
            incomplete
                .into_iter()
                .for_each(|i| { tasks[i].render(ui); });
            // Check for removal
            let mut remove_at: Option<usize> = None;
            for (i, _) in tasks.iter().enumerate().filter(|(_, task)| task.remove_me ) {
                remove_at = Some(i);
                break;
            }
            if let Some(index) = remove_at {
                tasks.remove(index);
            }
        }
    }).response
}

#[derive(Serialize, Deserialize)]
pub struct TaskTree {
    name: String,
    unique_id: Id,
    creation_time: DateTime<Utc>,
    modified_time: DateTime<Utc>,
    tasks: TaskList,
}

#[derive(Serialize, Deserialize)]
pub struct TaskApp {
    trees: Vec<TaskTree>,
    #[serde(skip)]
    new_task_popup: NewTaskPopupButton,
}

impl Default for NewTaskPopupButton {
    fn default() -> Self {
        Self {
            title: "".to_owned(),
            description: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewTaskPopupButton {
    // title, description
    pub title: String,
    pub description: Option<String>,
}

impl NewTaskPopupButton {

    const NAME: &'static str = "new_task--popup";

    pub fn show(&mut self, ui: &mut Ui, text: impl Into<WidgetText>, list: &mut TaskList) -> Response {

        ui.menu_button(text.into(), |ui| {

            // Set up style
            ui.style_mut().visuals.window_rounding = Rounding::none();
            ui.style_mut().visuals.popup_shadow = Shadow::default();
            ui.style_mut().visuals.widgets.active.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.inactive.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.hovered.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.open.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.noninteractive.rounding = Rounding::none();


            /// empty means that the description is None.
            // underline format
            let ul_fmt = TextFormat { underline: Stroke::new(1.0, Color32::WHITE), ..Default::default() };
            // default format
            let def_fmt = TextFormat::default();
            // Add button text
            let mut add_button_text = text::LayoutJob::default();
            add_button_text.append("A", 0.0, ul_fmt.clone());
            add_button_text.append("dd", 0.0, TextFormat::default());
            let mut clear_button_text = text::LayoutJob::default();
            clear_button_text.append("C", 0.0, ul_fmt.clone());
            clear_button_text.append("lear", 0.0, TextFormat::default());
            // Add/Remove Description button text
            let mut description_button_text = text::LayoutJob::default();
            if self.description.is_none() {
                description_button_text.append("Add ", 0.0, TextFormat::default());
                description_button_text.append("D", 0.0, ul_fmt.clone());
                description_button_text.append("escription ", 0.0, TextFormat::default());
            } else {
                description_button_text.append("Remove ", 0.0, TextFormat::default());
                description_button_text.append("D", 0.0, ul_fmt.clone());
                description_button_text.append("escription ", 0.0, TextFormat::default());
            }
            // group frame and vertical layout
            ui.group(|ui| { ui.vertical(|ui| {
                // Check for Alt+A
                if !self.title.is_empty()
                && ui.ctx().input_mut().consume_key(Modifiers::ALT, Key::A) {
                    let mut task = Task::new(self.title.clone());
                    self.title.replace("");
                    task.description = self.description.take();
                    list.push(task);
                }
                // Check for Alt+C
                if ui.ctx().input_mut().consume_key(Modifiers::ALT, Key::C) {
                    self.title.replace("");
                    self.description = None;
                    ui.memory().request_focus(Id::new(Self::NAME).with(Id::new("title_text")));
                }
                // Check for Alt+D
                if ui.ctx().input_mut().consume_key(Modifiers::ALT, Key::D) {
                    if self.description.is_none() {
                        self.description = Some(String::default());
                        ui.memory().request_focus(Id::new(Self::NAME).with(Id::new("description_text")));
                    } else {
                        self.description = None;
                        ui.memory().request_focus(Id::new(Self::NAME).with(Id::new("title_text")));
                    }
                }
                // let title_focused = ui
                //     .ctx()
                //     .data()
                //     .get_persisted(Id::new(Self::NAME))
                //     .unwrap_or(false);
                if !self.title.is_empty()
                && ui.ctx().input_mut().consume_key(Modifiers::CTRL, Key::Enter) {
                    let mut task = Task::new(self.title.clone());
                    self.title.replace("");
                    task.description = self.description.take();
                    list.push(task);
                }
                ui.allocate_ui(vec2(300.0, ui.spacing().interact_size.y), |ui| {
                    let title_text = ui.put(
                        ui.max_rect(),
                        TextEdit::singleline(&mut self.title)
                            .hint_text("Title")
                            .id(Id::new(Self::NAME).with(Id::new("title_text")))
                            .font(FontSelection::Style(TextStyle::Monospace))
                    );
                    if title_text.lost_focus()
                    && !self.title.is_empty()
                    && ui.ctx().input().key_pressed(Key::Enter) {
                        let mut task = Task::new(self.title.clone());
                        self.title.replace("");
                        task.description = self.description.take();
                        list.push(task);
                        ui.memory().request_focus(Id::new(Self::NAME).with(Id::new("title_text")));
                    }
                });
                ui.allocate_ui(vec2(ui.available_width(), ui.spacing().interact_size.y * 4.0), |ui| {
                    let description_text = self.description.as_mut().and_then(|description| {
                        Some(ui.put(
                            ui.max_rect(),
                            TextEdit::multiline(description)
                                .hint_text("Description")
                                .desired_rows(4)
                                .id(Id::new(Self::NAME).with(Id::new("description_text")))
                                .font(FontSelection::Style(TextStyle::Monospace))
                                .lock_focus(true)
                        ))
                    });
                    if let Some(description_edit) = description_text {
                        if description_edit.has_focus() && ui.ctx().input_mut().consume_key(Modifiers::CTRL, Key::Enter) {
                            // This should cause a task to be added.
                            print_unimplemented_message!("Ctrl+Enter in Description Text Edit.");
                        }
                    }
                });
                ui.allocate_ui(vec2(ui.available_width(), ui.spacing().interact_size.y), |ui| {
                    if !self.title.is_empty() {
                        ui.columns(2, |columns| {
                            function_bar!(
                                columns[0];
                                add_button_text => {
                                    let mut task = Task::new(self.title.clone());
                                    self.title.replace("");
                                    task.description = self.description.take();
                                    list.push(task);
                                }
                                clear_button_text => {
                                    self.title.replace("");
                                    self.description = None;
                                }
                            );
                            let description_btn = columns[1].put(columns[1].max_rect(), Button::new(description_button_text));
                            if description_btn.clicked()
                            || columns[1].ctx().input_mut().consume_key(Modifiers::CTRL, Key::D) {
                                if self.description.is_some() {
                                    self.description = None;
                                } else {
                                    self.description = Some(String::default());
                                }
                            }
                        });
                    } else {
                        let description_btn = ui.put(ui.max_rect(), Button::new(description_button_text));
                        if description_btn.clicked()
                        || ui.ctx().input_mut().consume_key(Modifiers::CTRL, Key::D) {
                            if self.description.is_some() {
                                self.description = None;
                            } else {
                                self.description = Some(String::default());
                            }
                        }
                    }
                });
            });});
        }).response
    }
}