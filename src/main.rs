mod scheduler;

use crate::scheduler::{Task, Tasks};

use std::rc::Rc;
use std::cell::RefCell;

use libadwaita::prelude::*;

use libadwaita::{ActionRow, Application, ApplicationWindow, HeaderBar, EntryRow};
use gtk::{ListBox, Orientation, SelectionMode, Button};
use gtk::glib;

fn create_row(task: &Task, list: &ListBox, tasks: Rc<RefCell<Tasks>>) -> ActionRow {
    let row = ActionRow::builder()
        .title(&task.name)
        .subtitle(format!("Priority: {}", task.priority))
	.build();
    let button = Button::builder()
        .icon_name("edit-delete")
        .css_classes(vec!["flat"])
        .build();
    row.add_suffix(&button);
    button.connect_clicked(glib::clone!(@strong task, @strong tasks, @weak list => move |_| {
	tasks.borrow_mut().remove(task.clone());
	build_list_from_tasks(&list, tasks.clone());
    }));
    row
}

fn build_list_from_tasks(list: &ListBox, tasks: Rc<RefCell<Tasks>>) {
    list.remove_all();
    for task in &tasks.borrow().tasks {
	list.append(&create_row(task, list, tasks.clone()));
    }
}

fn build_ui(app: &Application) {
    let tasks = Rc::new(RefCell::new(Tasks::load()));
    
    let list = ListBox::builder()
	.margin_top(32)
	.margin_end(32)
	.margin_bottom(32)
	.margin_start(32)
	.selection_mode(SelectionMode::None)
	.css_classes(vec!["boxed-list"])
	.build();
    build_list_from_tasks(&list, tasks.clone());

    let add_button = Button::builder().icon_name("list-add").build();
    let step_button = Button::builder().icon_name("edit-redo").build();
    let step_and_finish_button = Button::builder().icon_name("emblem-ok").build();
    
    let header_bar = HeaderBar::builder()
        .build();
    header_bar.pack_start(&add_button);
    header_bar.pack_start(&step_button);
    header_bar.pack_start(&step_and_finish_button);

    let content = gtk::Box::new(Orientation::Vertical, 0);
    content.append(&header_bar);
    content.append(&list);

    let window = ApplicationWindow::builder()
	.application(app)
	.title("Nasin")
	.content(&content)
	.build();

    let add_task_title_row = EntryRow::builder()
        .title("Name")
        .build();
    let add_task_priority_row = EntryRow::builder()
	.title("Priority")
	.input_purpose(gtk::InputPurpose::Digits)
	.build();
    let add_task_create_button = Button::builder()
        .label("Create Task")
        .margin_bottom(12)
        .margin_start(12)
        .css_classes(vec!["suggested-action"])
        .margin_end(12)
	.build();
    let add_task_content = gtk::ListBox::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .css_classes(vec!["boxed-list"])
	.build();
    add_task_content.append(&add_task_title_row);
    add_task_content.append(&add_task_priority_row);

    let holder_box = gtk::Box::new(Orientation::Vertical, 0);
    holder_box.append(&HeaderBar::new());
    holder_box.append(&add_task_content);
    holder_box.append(&add_task_create_button);

    let add_task_window = ApplicationWindow::builder()
        .title("Add Task")
        .modal(true)
        .transient_for(&window)
        .content(&holder_box)
        .hide_on_close(true)
        .build();

    add_button.connect_clicked(glib::clone!(@weak add_task_window => move |_| {
	add_task_window.present();
    }));

    step_button.connect_clicked(glib::clone!(@strong tasks, @weak list => move |_| {
	tasks.borrow_mut().step();
	build_list_from_tasks(&list, tasks.clone());
    }));

    step_and_finish_button.connect_clicked(glib::clone!(@strong tasks, @weak list => move |_| {
	tasks.borrow_mut().step_and_finish();
	build_list_from_tasks(&list, tasks.clone())
    }));

    add_task_create_button.connect_clicked(move |_| {
	let name: String = add_task_title_row.text().to_string();
	let priority: u8 = add_task_priority_row.text().parse().unwrap_or(1);
	if priority >= 1 {
	    tasks.borrow_mut().add(Task::new(name, priority));
	    build_list_from_tasks(&list, tasks.clone());
	    add_task_window.close();
	}
	// Reset the text of the rows
	add_task_title_row.set_text("");
	add_task_priority_row.set_text("");
    });
    window.present();
}

fn main() {
    let application = Application::builder()
	.application_id("me.jonot.Nasin")
	.build();

    application.connect_activate(build_ui);

    application.run();
}
