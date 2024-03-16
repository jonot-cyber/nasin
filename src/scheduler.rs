use std::{cmp::Ordering, fs::File, path::PathBuf};

use serde::{Serialize, Deserialize};

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

#[derive(Clone,Serialize,Deserialize)]
pub struct Task {
    pub name: String,
    pub priority: u8,
    age: u32,
    base_priority: u8,
}

impl Task {
    pub fn new(name: String, priority: u8) -> Task {
	Task {
	    name,
	    priority,
	    base_priority: priority,
	    age: 0,
	}
    }

    pub fn age(&mut self) {
	self.age = 0;
	self.priority = std::cmp::max(self.priority - 1, 1);
    }

    pub fn reset(&mut self) {
	self.age = 0;
	self.priority = self.base_priority;
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age && self.priority == other.priority && self.base_priority == other.base_priority
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
	    x => x
	}
    }
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>
}

impl Tasks {
    pub fn new() -> Self {
	Tasks {
	    tasks: Vec::new()
	}
    }

    pub fn load() -> Self {
	let path = get_data_path();
	let file = File::options()
	    .read(true)
	    .open(path)
	    .unwrap();
	if file.metadata().unwrap().len() == 0 {
	    Tasks::new()
	} else {
	    serde_json::from_reader(file).unwrap()
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
	    return
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
	    return
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
}
