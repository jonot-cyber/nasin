mod add;

use std::{cell::RefCell, io};

use nasin::scheduler::{Task, Tasks};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Constraint,
    prelude::{Buffer, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Row, Table, Widget},
    DefaultTerminal, Frame,
};

struct App<'a> {
    tasks: Tasks,
    selected: usize,
    exit: bool,
    add_popup_open: bool,
    add_popup: RefCell<add::Popup<'a>>,
}

impl App<'_> {
    pub fn new() -> Self {
        App {
            tasks: Tasks::load(),
            selected: 0,
            exit: false,
            add_popup_open: false,
            add_popup: RefCell::new(add::Popup::new()),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.add_popup_open {
            match key_event.code {
                KeyCode::Esc => {
                    self.add_popup_open = false;
                    self.add_popup.borrow_mut().reset()
                },
                KeyCode::Enter => {
                    if let Some(task) = self.add_popup.borrow().to_task() {
                        self.tasks.add(task);
                    }
                    self.add_popup.borrow_mut().reset();
                    self.add_popup_open = false
                }
                KeyCode::Tab | KeyCode::Down => self.add_popup.borrow_mut().focus_down(),
                KeyCode::BackTab | KeyCode::Up => self.add_popup.borrow_mut().focus_up(),
                _ => self.add_popup.borrow_mut().handle_key_event(key_event),
            }
        } else {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                KeyCode::Char('j') | KeyCode::Down => self.select_down(),
                KeyCode::Char('k') | KeyCode::Up => self.select_up(),
                KeyCode::Char('s') => self.step(),
                KeyCode::Char('f') => self.finish(),
                KeyCode::Char('p') => self.pause(),
                KeyCode::Char('d') => self.remove(),
                KeyCode::Char('a') => self.add_popup_open = true,
                _ => {}
            }
        }
    }

    // Move selection down
    fn select_down(&mut self) {
        let len = self.tasks.tasks.len();
        self.selected = (self.selected + 1).min(len - 1)
    }

    // Move selection down
    fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1)
    }

    // Step the tasks
    fn step(&mut self) {
        self.tasks.step();
    }

    // Finish the current task
    fn finish(&mut self) {
        self.tasks.step_and_finish();
    }

    // Toggle a task's paused state
    fn pause(&mut self) {
        let task = &self.tasks.tasks[self.selected];
        self.tasks.toggle_pause(&task.clone());
    }

    // Remove fa task
    fn remove(&mut self) {
        let task = &self.tasks.tasks[self.selected];
        self.tasks.remove(task.clone());
    }
}

fn task_to_row(task: &Task, highlight: bool) -> Row {
    let highlight_style = Style::new().fg(Color::Black).bg(Color::LightYellow);
    let paused_str = if task.paused { "[P]" } else { "[ ]" };
    let deadline_str = if let Some(date) = task.deadline {
        date.format("%Y-%m-%d").to_string()
    } else {
        String::from("-")
    };
    let row = Row::new(vec![
        Cell::new(paused_str),
        task.name.clone().into(),
        task.priority.to_string().into(),
        deadline_str.into(),
    ]);
    if highlight {
        row.style(highlight_style)
    } else {
        row
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.add_popup_open {
            let title = Line::from(" Add Task... ".bold());
            let instructions = Line::from(vec![
                " Next ".into(),
                "<Tab>".blue().bold(),
                " Previous ".into(),
                "<S-Tab>".blue().bold(),
                " Add ".into(),
                "<Enter>".blue().bold(),
                " Quit ".into(),
                "<Esc> ".blue().bold(),
            ]);
            let block = Block::bordered()
                .title(title.centered())
                .title_bottom(instructions)
                .border_set(border::THICK);
            self.add_popup.borrow_mut().render(block.inner(area), buf);
            block.render(area, buf);
        } else {
            let title = Line::from(" Nasin ".bold());
            let instructions = Line::from(vec![
                " Down ".into(),
                "<j/Down>".blue().bold(),
                " Up ".into(),
                "<k/Up>".blue().bold(),
                " Step ".into(),
                "<s>".blue().bold(),
                " Finish ".into(),
                "<f>".blue().bold(),
                " Toggle Pause ".into(),
                "<p>".blue().bold(),
                " Remove ".into(),
                "<d>".blue().bold(),
                " Add ".into(),
                "<a>".blue().bold(),
                " Quit ".into(),
                "<q/Esc> ".blue().bold(),
            ]);
            let block = Block::bordered()
                .title(title.centered())
                .title_bottom(instructions)
                .border_set(border::THICK);
            let header = Row::new(vec![
                "P?".bold(),
                "Name".bold(),
                "Priority".bold(),
                "Deadline".bold(),
            ]);
            let rows = self
                .tasks
                .tasks
                .iter()
                .enumerate()
                .map(|(i, t)| task_to_row(t, i == self.selected));
            Table::new(
                rows,
                vec![
                    Constraint::Length(3),
                    Constraint::Fill(2),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ],
            )
            .header(header)
            .block(block)
            .render(area, buf);
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
