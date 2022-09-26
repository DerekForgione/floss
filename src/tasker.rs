use std::time::Instant;

use egui::{self, *};

/// TODO
///  Create clocked task. The clocked task will have a start/stop button which will measure the
///     amount of time spent on a task.  
//////

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

/// I know the name is strange, but this is meant to represent
/// the two types of tasks.
#[derive(serde::Deserialize, serde::Serialize)]
pub enum TaskData {
    Single(bool),
    Multiple(Vec<Task>),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum ControlMode {
    Default,
    Editing,
}

impl Default for ControlMode {
    fn default() -> Self {
        ControlMode::Default
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub title: String,
    /// Description is optional. Can be viewed as popup.
    pub description: Option<String>,
    pub id: Id,
    pub tag: usize,    
    pub data: TaskData,
    #[serde(skip)]
    remove_me: bool,
    #[serde(skip)]
    mode: ControlMode,
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
            mode: ControlMode::Default,
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
            mode: ControlMode::Default,
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
            let mut check = self.complete();
            let mark = ui.checkbox(&mut check, "");
            
            // if check && self.incomplete() {
            //     self.mark_complete();
            // } else if !check && self.complete() {
            //     self.mark_incomplete();
            // }
            let title = if self.mode == ControlMode::Editing {
                let resp = ui.text_edit_singleline(&mut self.title);
                if resp.lost_focus() {
                    self.mode = ControlMode::Default;
                }
                resp
            } else {
                let resp = ui.label(self.title.as_str()).interact(Sense::click());
                if resp.clicked() {
                    self.mode = ControlMode::Editing;
                }
                resp
            };
            let right = button::right(ui, self);
            if mark.changed() {
                self.toggle_complete();
            }
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
                        render_list(ui, tasks, AdderPosition::Above);
                    }).0
            }
        }
        
    }
    
}

enum AddEvent {
    None,
    AddTask,
    AddGroup,
}

trait Respond {
    fn respond(&mut self, response: Response) -> Response;
    fn render<F>(&mut self, ui: &mut Ui, renderer: F) -> Response
    where F: FnOnce(&mut Ui) -> Response {
        self.respond(renderer(ui))
    }
}

impl Respond for Option<Response> {
    /// Sets or unions self, returns the response input.
    /// This allows you to apply a response while also reacting to it.
    fn respond(&mut self, response: Response) -> Response {
        if let Some(me) = self {
            *me = me.union(response.clone());
        } else {
            *self = Some(response.clone());
        }
        response
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Eq, Ord)]
pub enum AdderPosition {
    None,
    Above,
    Below,
    Both,
}

impl AdderPosition {
    fn above(&self) -> bool {
        match *self {
            AdderPosition::Above | AdderPosition::Both => true,
            _ => false,
        }
    }

    fn below(&self) -> bool {
        match *self {
            AdderPosition::Below | AdderPosition::Both => true,
            _ => false,
        }
    }
}

pub fn render_list(ui: &mut Ui, tasks: &mut Vec<Task>, adders: AdderPosition) -> Response {
    ui.vertical(|ui| {

        if adders.above() { button::adders(ui, tasks); }

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
        
        if adders.below() { button::adders(ui, tasks); }

    }).response
}

mod button {
    use super::*;

    pub fn adders(ui: &mut Ui, tasks: &mut Vec<Task>) -> Response {
        let (mut response, action) = ui.horizontal(|ui| {
            let task = ui.small_button("Add Task");
            let group = ui.small_button("Add Group");
            let action = if task.clicked() {
                AddEvent::AddTask
            } else if group.clicked() {
                AddEvent::AddGroup
            } else {
                AddEvent::None
            };
            (task | group, action)
        }).inner;
        match action {
            AddEvent::AddTask => {
                tasks.push(Task::new("Untitled Task", ui.next_id()));
                response.mark_changed();
            },
            AddEvent::AddGroup => {
                tasks.push(Task::group("Untitled Group", ui.next_id()).with(vec![Task::new("Untitled Task", ui.next_id())]));
                response.mark_changed();
            },
            _ => (),
        }
        response
    }

    pub fn right(ui: &mut Ui, task: &mut Task) -> Response {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            let exit = ui.small_button(RichText::new("⊗").color(Color32::RED));
            if exit.clicked() {
                task.remove_me = true;
            }
            let edit = ui.small_button(if task.mode == ControlMode::Default { "✏" } else { "💾" });
            if edit.clicked() {
                task.mode = match  task.mode {
                    ControlMode::Default => ControlMode::Editing,
                    ControlMode::Editing => ControlMode::Default,
                };
            }
            exit | edit
        }).inner
    }

    pub fn mark(ui: &mut Ui, incomplete: bool) -> Response {
        ui.small_button(
            if incomplete { RichText::new("Done").color(Color32::GREEN) }
            else { RichText::new("Recycle").color(Color32::RED) }
        )
    }

}