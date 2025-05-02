use chrono::{Local, NaiveDate, NaiveDateTime};
use nasin::scheduler::Task;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::StatefulWidget,
    prelude::{Buffer, Rect},
    style::Stylize,
    widgets::{Paragraph, Widget},
};

use tui_widgets::prompts::{State, TextPrompt, TextState};

#[derive(Default)]
pub struct Popup<'a> {
    current: FocusState,
    name: TextState<'a>,
    priority: TextState<'a>,
    date: TextState<'a>,
}

#[derive(Default)]
enum FocusState {
    #[default]
    Name,
    Priority,
    Date,
}

impl Popup<'_> {
    pub fn new() -> Self {
        Self {
            current: FocusState::default(),
            name: TextState::new().with_focus(tui_widgets::prompts::FocusState::Focused),
            priority: TextState::new(),
            date: TextState::new(),
        }
    }

    pub fn reset(&mut self) {
        self.current = FocusState::default();
        self.name.value_mut().clear();
        self.priority.value_mut().clear();
        self.date.value_mut().clear();
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(1); 3])
            .split(area);
        TextPrompt::new("Name".into()).render(layout[0], buf, &mut self.name);
        TextPrompt::new("Priority".into()).render(layout[1], buf, &mut self.priority);
        TextPrompt::new("Date".into()).render(layout[2], buf, &mut self.date);
    }

    pub fn focus_down(&mut self) {
        self.current = match self.current {
            FocusState::Name => FocusState::Priority,
            FocusState::Priority => FocusState::Date,
            FocusState::Date => FocusState::Date,
        }
    }

    pub fn focus_up(&mut self) {
        self.current = match self.current {
            FocusState::Name => FocusState::Name,
            FocusState::Priority => FocusState::Name,
            FocusState::Date => FocusState::Priority,
        }
    }

    pub fn handle_key_event(&mut self, key_event: ratatui::crossterm::event::KeyEvent) {
        match &self.current {
            FocusState::Name => self.name.handle_key_event(key_event),
            FocusState::Priority => self.priority.handle_key_event(key_event),
            FocusState::Date => self.date.handle_key_event(key_event),
        }
    }

    pub fn to_task(&self) -> Option<Task> {
        let priority: u8 = self.priority.value().parse().unwrap_or(1);
        let deadline_str = self.date.value();
        let deadline = if deadline_str.is_empty() {
            None
        } else {
            let date = NaiveDate::parse_from_str(deadline_str, "%Y-%m-%d").ok()?;
            let datetime: NaiveDateTime = date.into();
            Some(datetime.and_local_timezone(Local).unwrap())
        };
        Some(Task::new(
            String::from(self.name.value()),
            priority,
            deadline,
        ))
    }
}
