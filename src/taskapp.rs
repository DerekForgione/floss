use std::ops::Index;

use egui::{self, *};

/// TODO
/// Clock Task
///     Allows you to clock in/out to record time
//////

pub enum TaskData<'a> {
    Single(bool),
    Multi(Vec<TaskItem<'a>>),
}

fn next_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub struct TaskItem<'a> {
    title: String,
    description: Option<String>,
    data: TaskData<'a>,
    id: usize,
    parent: &'a mut Vec<TaskItem<'a>>,
}

pub struct TaskApp {

}

impl<'a> TaskData<'a> {
    fn single() -> Self {
        TaskData::<'a>::Single(false)
    }

    fn multi() -> Self {
        TaskData::<'a>::Multi(Vec::new())
    }
}

impl<'a> TaskItem<'a> {
    pub fn index(&mut self) -> Option<usize> {
        for (i, v) in self.parent.iter().enumerate() {
            if v.id == self.id {
                return Some(i);
            }
        }
        None
    }

    pub fn delete(mut self) {
        if let Some(index) = self.index() {
            self.parent.remove(index);
        }
    }

    pub fn new<T: Into<String>>(title: T, parent: &'a mut Vec<TaskItem<'a>>) -> Self {
        Self {
            title: title.into(),
            description: None,
            data: TaskData::Single(false),
            id: next_id(),
            parent,
        }
    }

}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
                        
        });
    }
}