use std::sync::mpsc::Sender;

use ratatui::{
    crossterm::event::KeyCode, layout::{Constraint, Layout, Rect}, style::Stylize, widgets::{Block, Borders, Paragraph}, Frame
};

use crate::{config::TermConfig, typie::TypieEvent, utils::center};

pub struct Test<'a> {
    term_config: &'a TermConfig,
    tx: Sender<TypieEvent>
}

impl<'a> Test<'a> {
    pub fn new(term_config: &'a TermConfig, tx: Sender<TypieEvent>) -> Self {
        Self {
            term_config,
            tx
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.tx.send(TypieEvent::ChangeScreen(super::Screens::MainMenu)).unwrap(),
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

        for (i, letter) in ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"]
            .iter()
            .enumerate()
        {
            frame.render_widget(
                Paragraph::new(letter.to_string())
                    .bold()
                    .centered()
                    .block(Block::new().borders(Borders::ALL)),
                Rect::new(bottom.x + i as u16 * 6, bottom.y, 5, 3),
            );
        }

        for (i, letter) in ["A", "S", "D", "F", "G", "H", "J", "K", "L"]
            .iter()
            .enumerate()
        {
            frame.render_widget(
                Paragraph::new(letter.to_string())
                    .bold()
                    .centered()
                    .block(Block::new().borders(Borders::ALL)),
                Rect::new(bottom.x + 3 + i as u16 * 6, bottom.y + 3, 5, 3),
            );
        }

        for (i, letter) in
            ["Z", "X", "C", "V", "B", "N", "M"].iter().enumerate()
        {
            frame.render_widget(
                Paragraph::new(letter.to_string())
                    .bold()
                    .centered()
                    .block(Block::new().borders(Borders::ALL)),
                Rect::new(bottom.x + 9 + i as u16 * 6, bottom.y + 6, 5, 3),
            );
        }

        frame.render_widget(
            Paragraph::new("")
                .bold()
                .centered()
                .block(Block::new().borders(Borders::ALL)),
            Rect::new(bottom.x + 11, bottom.y + 9, 36, 3),
        );
    }
}
