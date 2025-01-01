use std::sync::mpsc::Sender;

use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    config::{Config, TermConfig},
    typie::TypieEvent,
    utils::center,
};

pub struct Test<'a> {
    config: &'a Config,
    term_config: &'a TermConfig,
    tx: Sender<TypieEvent>,
}

impl<'a> Test<'a> {
    pub fn new(
        config: &'a Config,
        term_config: &'a TermConfig,
        tx: Sender<TypieEvent>,
    ) -> Self {
        Self {
            config,
            term_config,
            tx,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self
                .tx
                .send(TypieEvent::ChangeScreen(super::Screens::MainMenu))
                .unwrap(),
            _ => {}
        }
    }

    pub fn draw(&self, frame: &mut Frame<'_>) {
        let area = center(
            Constraint::Length(self.term_config.width),
            Constraint::Length(self.term_config.height),
            frame.area(),
        );

        let [top, bottom] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .areas(area);

        let top = center(
            Constraint::Length(self.term_config.width),
            Constraint::Length(3),
            top,
        );

        frame.set_cursor_position(top.as_position());

        frame.render_widget(
            Paragraph::new("test foo"),
            Rect::new(top.x, top.y, self.term_config.width, 1),
        );

        frame.render_widget(
            Paragraph::new("test foo"),
            Rect::new(top.x, top.y + 1, self.term_config.width, 1),
        );

        frame.render_widget(
            Paragraph::new("test foo"),
            Rect::new(top.x, top.y + 2, self.term_config.width, 1),
        );

        if self.config.show_keyboard {
            self.render_keyboard(frame, &bottom);
        }
    }

    fn render_keyboard(&self, frame: &mut Frame<'_>, area: &Rect) {
        self.render_row_keyboard(
            frame,
            vec!["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
            area,
            0,
            0,
        );

        self.render_row_keyboard(
            frame,
            vec!["A", "S", "D", "F", "G", "H", "J", "K", "L"],
            area,
            3,
            3,
        );

        self.render_row_keyboard(
            frame,
            vec!["Z", "X", "C", "V", "B", "N", "M"],
            area,
            9,
            6,
        );

        // Spacebar
        frame.render_widget(
            Block::new().borders(Borders::ALL),
            Rect::new(area.x + 11, area.y + 9, 36, 3),
        );
    }

    fn render_row_keyboard(
        &self,
        frame: &mut Frame<'_>,
        letters: Vec<&str>,
        area: &Rect,
        x_offset: u16,
        y_offset: u16,
    ) {
        for (i, letter) in letters.iter().enumerate() {
            frame.render_widget(
                Paragraph::new(letter.to_string())
                    .bold()
                    .centered()
                    .block(Block::new().borders(Borders::ALL)),
                Rect::new(
                    area.x + x_offset + i as u16 * 6,
                    area.y + y_offset,
                    5,
                    3,
                ),
            );
        }
    }
}
