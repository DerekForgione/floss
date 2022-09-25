// it feels weird to start code on the first line of the file

use egui::{self, *};

pub struct SingleTask {
    title: String,
    description: Option<String>,
    completed: bool,
}

pub struct TaskGroup {
    title: String,
    description: Option<String>,
    
}

// So my basic idea is that there would be multiple different types of
// tasks, each of which can be represented as a widget.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum Task {
    Single { title: String, description: Option<String>, completed: bool },
    // Group is considered to be completed when all of its child tasks are completed.
    Group { title: String, description: Option<String>, tasks: Vec<Task> },
}

impl Task {

    fn mark_complete(&mut self) {
        match self {
            Task::Single { completed, .. } => *completed = true,
            Task::Group { tasks, .. } => {
                for task in &mut tasks.iter_mut() {
                    task.mark_complete();
                }
            }
        }
    }

    fn mark_incomplete(&mut self) {
        match self {
            Task::Single { completed, .. } => *completed = false,
            Task::Group { tasks, .. } => {
                for task in &mut tasks.iter_mut() {
                    task.mark_incomplete();
                }
            }
        }
    }

    fn completed(&self) -> bool {
        match  self {
            Self::Single { completed,.. } => *completed,
            Self::Group { tasks,.. } => {
                if tasks.len() == 0 {
                    return true;
                }
                let mut completed = true;
                tasks.into_iter().for_each(|task| completed &= task.completed());
                completed
            }
        }
    }

    fn incomplete(&self) -> bool {
        match self {
            Task::Single { completed, .. } => !*completed,
            Task::Group { tasks, .. } => {
                if tasks.len() != 0 {
                    for task in tasks.iter() {
                        if task.incomplete() {
                            return true;
                        }
                    }
                }
                false
            }
        }
    }

    fn total_tasks(&self) -> usize {
        match self {
            Task::Single { .. } => 1,
            Task::Group { tasks, .. } => {
                let mut count = 0usize;
                tasks.iter().for_each(|v| count += v.total_tasks() );
                count
            }
        }
    }

    fn total_completed_tasks(&self) -> usize {
        match self {
            Task::Single { completed, .. } => if *completed { 1 } else { 0 },
            Task::Group { tasks, .. } => {
                let mut count = 0usize;
                tasks.iter().for_each(|v| count += v.total_completed_tasks() );
                count
            }
        }
    }

    fn completion_percent(&self) -> f64 {
        // completed / total 
        let total = self.total_tasks();
        if total == 0 {
            return 0.0;
        }
        (self.total_completed_tasks() / self.total_tasks()) as f64 * 100.0
    }

    fn new(title: String) -> Task {
        Task::Single { title: title, description: None, completed: false }
    }

    fn title(&self) -> &str {
        match self {
            Task::Single { title, .. } => title.as_str(),
            Task::Group { title, .. } => title.as_str(),
        }
    }

    fn description(&self) -> Option<&str> {
        let opt = match &self {
            Task::Single { description, .. } => description,
            Task::Group { description, .. } => description,
        };
        if let Some(s) = opt {
            Some(s.as_str())
        } else {
            None
        }
    }

    fn set_description(&mut self, text: String) {
        match self {
            Self::Single { description, .. } => *description = Some(text),
            Self::Group { description, .. } => *description = Some(text),
        }
    }

    fn add_task(&mut self, task: Task) -> Result<(),&str> {
        if let Task::Group { tasks, .. } = self {
            tasks.push(task);
            Ok(())
        } else {
            Err("Not a task group.")
        }
    }

}

#[derive(serde::Deserialize, serde::Serialize)]
enum TaskListSort {
    InOrder,
    ByCompletion,
}

impl Default for TaskListSort {
    fn default() -> Self { TaskListSort::InOrder }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct TaskListOptions {
    sort: TaskListSort,
}

impl Default for TaskListOptions {
    fn default() -> Self {
        Self { 
            sort: TaskListSort::InOrder,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TaskList {
    tasks: Vec<Task>,
    options: TaskListOptions,
}

impl Default for TaskList {
    fn default() -> Self {
        Self { tasks: Default::default(), options: Default::default() }
    }
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Default::default(),
            options: Default::default(),
        }
    }

    pub fn sorted() -> Self {
        Self::new().options(TaskListOptions { sort: TaskListSort::InOrder })
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn with(mut self, tasks: Vec<Task>) -> Self {
        self.tasks = tasks;
        self
    }

    pub fn options(mut self, options: TaskListOptions) -> Self {
        self.options = options;
        self
    }

}

struct SubTask<'a> {
    root: &'a mut TaskList,
    parent: &'a mut Vec<Task>,
    task: &'a mut Task,
    index: usize,
    remove_me: bool,
}

impl<'a> SubTask<'a> {
    fn new(root: &'a mut TaskList, parent: &'a mut Vec<Task>, task: &'a mut Task, index: usize) -> Self {
        Self {
            root,
            parent,
            task,
            index,
            remove_me: false,
        }
    }
}

impl<'a> Widget for &mut SubTask<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        
        let header = |ui: &mut Ui, task: &mut SubTask<'_>| {
            ui.horizontal(|ui| {
                
                let resp = if task.task.incomplete() {
                    let mut done = ui.small_button("Done");
                    if done.clicked() {
                        task.task.mark_complete();
                        done.mark_changed();
                    }
                    done
                } else {
                    let mut done = ui.small_button(RichText::new("Recycle").color(Color32::RED));
                    if done.clicked() {
                        task.task.mark_incomplete();
                        done.mark_changed();
                    }
                    done
                };
                resp.union(ui.label(task.task.title()));
    
                resp.union(ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    let resp = ui.small_button(RichText::new("Remove").color(Color32::RED));
                    if resp.clicked() {
                        task.remove_me = true;
                    }
                    resp
                }).inner)
            }).inner
        };

        match self.task {
            Task::Single { .. } => {
                header(ui, self)
            }
            Task::Group { tasks, .. } => {
                let id = ui.make_persistent_id(self.task.title().as_str());
                collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
                    .show_header(ui, |ui| {
                        header(ui, self)
                    })
                    .body(|ui| {
                        let mut response: Option<Response> = None;
                        let respond = |resp| {
                            if let Some(response) = response {
                                response.union(resp)
                            } else {
                                response = Some(resp);
                                response.unwrap()
                            }
                        };
                        let mut remove_at: Option<usize> = None;
                        ui.vertical(|ui| {
                            match self.root.options.sort {                            
                                TaskListSort::InOrder => {
                                    tasks.iter_mut().enumerate().for_each(|(i, task)| {
                                        let mut sub = SubTask::new(self.root, &mut tasks, &mut task, i);
                                        ui.add(&mut sub);
                                        // Allows us to store the removal index for later.
                                        if sub.remove_me {
                                            remove_at = Some(i);
                                            if let Some(resp) = &mut response {
                                                resp.mark_changed();
                                            }
                                        }
                                    });
                                }   
                                TaskListSort::ByCompletion => {
                                    tasks.iter_mut()
                                        .enumerate()
                                        .filter(|(i, task)| { task.incomplete() })
                                        .for_each(|(i, task)| {
                                            let mut sub = SubTask::new(self.root, &mut tasks, &mut task, i);
                                            ui.add(&mut sub);
                                            // Allows us to store the removal index for later.
                                            if sub.remove_me {
                                                remove_at = Some(i);
                                            }
                                        });
                                    tasks.iter_mut()
                                        .enumerate()
                                        .filter(|(i, task)| { task.completed() })
                                        .for_each(|(i, task)| {
                                            let mut sub = SubTask::new(self.root, &mut tasks, &mut task, i);
                                            ui.add(&mut sub);
                                            // Allows us to store the removal index for later.
                                            if sub.remove_me {
                                                remove_at = Some(i);
                                            }
                                        });
                                }
                            }
                            // Removal of task
                            if let Some(index) = remove_at {
                                // TODO: Add popup to ask user if they are sure
                                tasks.remove(index);
                                if let Some(resp) = &mut response {
                                    resp.mark_changed();
                                }
                            }
                        })
                    }).0
            }
        }
    }
}

impl egui::Widget for &mut TaskList {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.group(|ui| {
            // First, the function bar
            // Add Task | Select Multiple
            let mut resp = ui.horizontal(|ui| {
                let mut add_btn = ui.small_button("Add Task");
                ui.label("|");
                let mut select_btn = ui.small_button("Select Multiple");

                if add_btn.clicked() {
                    self.tasks.push(Task::new("New Task".into()));
                }

                add_btn.union(select_btn)
            }).inner;
            // Task List Elements
            resp.union(ui.vertical(|ui| {
                if self.tasks.len() == 0 {
                    let mut resp = ui.hyperlink("No Tasks. Click To Add Task.");
                    if resp.clicked() {
                        self.tasks.push(Task::new("New Task".into()));
                        resp.mark_changed();
                    }
                    resp
                } else {
                    let mut resp: Option<Response> = None;
                    let respond = |response| {
                        if let Some(resp) = &mut resp {
                            resp.union(response)
                        } else {
                            resp = Some(response);
                            response
                        }
                    };
                    let mut remove_at: Option<usize> = None;
                    let parent: &mut Vec<Task> = &mut self.tasks;
                    let remove_btn = |ui: &mut Ui, index| {
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            let mut resp = ui.small_button(RichText::new("Remove").color(Color32::RED));
                            if resp.clicked() {
                                remove_at = Some(index);
                            }
                            resp
                        }).inner
                    };

                    match self.options.sort {
                        TaskListSort::ByCompletion => {
                            remove_at = None;
                            self.tasks.iter_mut().enumerate()
                                .filter(|(i, task)| task.incomplete())
                                .for_each(|(i, task)| {
                                    let mut sub = SubTask::new(self, &mut self.tasks, &mut task, i);
                                    respond(ui.add(&mut sub));
                                    if sub.remove_me {
                                        remove_at = Some(i);                                        
                                    }
                                });
                            self.tasks.iter_mut().enumerate()
                                .filter(|(i, task)| task.completed())
                                .for_each(|(i, task)| {
                                    let mut sub = SubTask::new(self, &mut self.tasks, &mut task, i);
                                    respond(ui.add(&mut sub));
                                    if sub.remove_me {
                                        remove_at = Some(i);                                        
                                    }
                                });
                            if let Some(index) = remove_at {
                                self.tasks.remove(index);
                            }
                        }
                        TaskListSort::InOrder => {
                            self.tasks.iter_mut().enumerate()
                                .for_each(|(i, task)| {
                                    let mut sub = SubTask::new(self, &mut self.tasks, &mut task, i);
                                    respond(ui.add(&mut sub));
                                    if sub.remove_me {
                                        remove_at = Some(i);                                        
                                    }
                                });
                            if let Some(index) = remove_at {
                                self.tasks.remove(index);
                            }
                        }
                    }

                    // TODO
                    resp.unwrap()
                }
            }).inner)
        }).inner
    }
}

impl egui::Widget for &mut Task {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        match self {
            Task::Single { title, description, completed } => {
                ui.horizontal(|ui| {
                    let mut resp = ui.small_button("Mark Complete");
                    if resp.clicked() {
                        *completed = true;
                        resp.mark_changed();
                    }
                    resp.union(ui.label(title.as_str()));
                    if let Some(text) = description {
                        let mut descbtn = ui.small_button("Show Description");
                        if descbtn.clicked() {
                            println!("Button pressed. [Task::Basic Small Button]");
                        }
                        resp.union(descbtn);
                    }
                    resp
                }).inner
            }
            Task::Group { title, tasks, .. } => {
                let id = ui.make_persistent_id(title.as_str());
                egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
                    .show_header(ui, |ui| {
                        let mut resp = ui.small_button("Done");
                        if resp.clicked() {
                            // Ask user if they are sure, and then if they are, mark all child
                            // tasks as completed.
                            println!("Mark Group Completed");
                        }
                        ui.label(title.as_str());
                        resp
                    })
                    .body(|ui| {
                        if tasks.len() == 0 {
                            let mut resp = ui.small_button("Add Task");
                            if resp.clicked() {
                                println!("Add Task");
                            }
                            resp
                        } else {
                            let mut remove_at: Option<usize> = None;
                            let mut resp: Option<Response> = None;
                            ui.group(|ui| {
                                let render = |(index, task)| {
                                    if let Some(resp) = &mut resp {
                                        resp.union(ui.add(task));
                                    } else {
                                        resp = Some(ui.add(task));
                                    }
                                };
                                tasks.iter_mut().enumerate().filter(|(_, it)| !it.completed()).for_each(render);
                                tasks.iter_mut().enumerate().filter(|(_, it)| it.completed()).for_each(render);
                                resp
                            }).response
                        }
                    }).0
            }
        }
    }
}