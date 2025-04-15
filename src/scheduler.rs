use std::{cmp::Ordering, fs::File, path::PathBuf};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

fn get_data_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push("nasin");
    if !path.exists() {
        std::fs::create_dir_all(path.clone()).unwrap();
    }
    path.push("tasks");
    path.set_extension("json");
    if !path.exists() {
        File::create(path.clone()).unwrap();
    }
    path
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub priority: u8,
    pub paused: bool,
    pub deadline: Option<DateTime<Local>>,
    age: u32,
    base_priority: u8,
}

impl Task {
    pub fn new(name: String, priority: u8, deadline: Option<DateTime<Local>>) -> Task {
        let new_priority = deadline
            .map(Task::priority_from_deadline)
            .unwrap_or(priority);
        Task {
            name,
            priority: new_priority,
            deadline,
            paused: false,
            base_priority: new_priority,
            age: 0,
        }
    }

    fn priority_from_deadline(deadline: DateTime<Local>) -> u8 {
        let diff = deadline - Local::now();
        diff.num_days().min(u8::MAX as i64).max(1) as u8
    }

    pub fn age(&mut self) {
        if !self.paused {
            self.age = 0;
            self.priority = std::cmp::max(self.priority - 1, 1);
        }
    }

    pub fn reset(&mut self) {
        self.age = 0;
        self.priority = self.base_priority;
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.paused == other.paused
            && self.name == other.name
            && self.age == other.age
            && self.priority == other.priority
            && self.base_priority == other.base_priority
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Task {}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.priority.cmp(&other.priority) {
            Ordering::Equal => other.age.cmp(&self.age),
            x => x,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks { tasks: Vec::new() }
    }

    pub fn load() -> Self {
        let path = get_data_path();
        let file = File::options().read(true).open(path).unwrap();
        if file.metadata().unwrap().len() == 0 {
            Tasks::new()
        } else {
            let mut ret: Tasks = serde_json::from_reader(file).unwrap();
            let mut priority_modified = false;
            for task in &mut ret.tasks {
                if let Some(date) = task.deadline {
                    let new_priority = Task::priority_from_deadline(date);
		    if task.base_priority != new_priority {
			task.base_priority = new_priority;
			priority_modified = true
		    }
                    task.base_priority = Task::priority_from_deadline(date);
                    task.priority = task.priority.min(task.base_priority)
                }
            }
	    if priority_modified {
		ret.tasks.sort();
	    }
            ret
        }
    }

    fn save(&self) {
        let path = get_data_path();
        let file = File::options()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        serde_json::to_writer(file, self).unwrap();
    }

    pub fn step(&mut self) {
        // Nothing to do if the list is empty
        if self.tasks.is_empty() {
            return;
        }
        self.tasks.sort();
        let mut current_task = self.tasks.remove(0);
        for task in self.tasks.iter_mut() {
            task.age += 1
        }
        let oldest_task = if self.tasks.len() == 1 {
            &mut self.tasks[0]
        } else {
            self.tasks
                .iter_mut()
                .filter(|x| !x.paused)
                .max_by_key(|x| x.age)
                .unwrap()
        };
        oldest_task.age();
        current_task.reset();
        self.tasks.push(current_task);
        self.tasks.sort();
        // Save after stepping
        self.save();
    }

    pub fn step_and_finish(&mut self) {
        // Nothing to do if the list is empty
        if self.tasks.is_empty() {
            return;
        }
        self.tasks.sort();
        self.tasks.remove(0);
        // Don't do anything else if we removed the last element
        if self.tasks.is_empty() {
            self.save();
            return;
        }
        for task in self.tasks.iter_mut() {
            task.age += 1
        }
        let oldest_task = self.tasks.iter_mut().max_by_key(|x| x.age).unwrap();
        oldest_task.age();
        self.tasks.sort();
        // Save after stepping
        self.save();
    }

    pub fn remove(&mut self, task: Task) {
        for (i, i_task) in self.tasks.iter().enumerate() {
            if task == *i_task {
                self.tasks.remove(i);
                self.save();
                return;
            }
        }
        // Save, but don't bother if we don't have anything here
        if self.tasks.is_empty() {
            self.save();
        }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
        self.tasks.sort();
        // Save after adding
        self.save();
    }

    pub fn toggle_pause(&mut self, task: &Task) {
        for t in &mut self.tasks {
            if *task == *t {
                t.paused = !t.paused
            }
        }
        self.save();
    }
}
