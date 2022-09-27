#![allow(unused)]

use std::time::Instant;

use egui::{self, *};

/// TODO
///  Create clocked task. The clocked task will have a start/stop button which will measure the
///     amount of time spent on a task.  
//////


pub type TaskList = Vec<Task>;

pub trait AtomicId {
    fn next_id(&self) -> Id;
    fn next_index(&self) -> usize;
}

impl AtomicId for Ui {

    fn next_id(&self) -> Id {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        self.id().with(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    fn next_index(&self) -> usize {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub id: Id,
    pub tag: usize,    
    pub data: TaskData,
    #[serde(skip)]
    remove_me: bool,
    #[serde(skip)]
    editing: bool,
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

impl Task {
    pub fn new<T: Into<String>>(title: T, id: Id) -> Self {
        Self {
            title: title.into(),
            description: None,
            id,
            tag: next_index(),
            data: TaskData::single(),
            remove_me: false,
            editing: false,
        }
    }

    pub fn group<T: Into<String>>(title: T, id: Id) -> Self {
        Self {
            title: title.into(),
            description: None,
            id,
            tag: next_index(),
            data: TaskData::multi(),
            remove_me: false,
            editing: false,
        }
    }

    pub fn with(mut self, tasks: Vec<Task>) -> Self {
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
        ui.horizontal(|ui| {
            let mark = ui.horizontal(|ui| {
                let mut check = self.complete();
                if self.editing {
                    ui.set_visible(false);
                }
                let mark = ui.checkbox(&mut check, "");
                if mark.changed() {
                    self.toggle_complete();
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
                let exit = ui.small_button(RichText::new("⊗").color(Color32::RED));
                if exit.clicked() {
                    self.remove_me = true;
                }
                let edit = ui.small_button(if self.editing { "✏" } else { "💾" });
                if edit.clicked() {
                    self.editing = !self.editing;
                }
                exit | edit
            }).inner;
            mark | title | right
        }).response
    }

    pub fn render(&mut self, ui: &mut Ui) -> Response {
        // This task will either be a single or a multi
        // TODO: Capture all Responses from ui to return to caller.
        let id = ui.make_persistent_id(self.id);
        match self.single() {
            true => {
                ui.push_id(id, |ui| self.render_header(ui)).inner
            }
            false => {
                collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
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
    
}

// pub fn render_task(ui: &mut Ui, task: &mut Task) -> Response {
//     ui.group(|group| {
//         // Render Header
//         if task.single() {
            
//         } else { // Multi   
//             ui.indent(task.id.with("sub_tasks"), |sub| {
                
//             });
//         }
//     }).inner
// }

pub fn render_list(ui: &mut Ui, tasks: &mut Vec<Task>) -> Response {
    ui.vertical(|ui| {

        if !tasks.is_empty() {
            // Order by incomplete tasks first.
            tasks.iter_mut()
                .filter(|task| task.incomplete() )
                .for_each(|task: &mut Task| {
                    task.render(ui);
                });
            tasks.iter_mut()
                .filter(|task| !task.incomplete() )
                .for_each(|task: &mut Task| {
                    task.render(ui);
                });
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