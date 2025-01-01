use std::{process, sync::mpsc::Sender};

use ratatui::{
    crossterm::{event::KeyCode, style::Color},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Text,
    Frame,
};

use crate::{config::TermConfig, typie::TypieEvent, utils::center};

use super::Screens;

pub struct MainMenu<'a> {
    term_config: &'a TermConfig,
    tx: Sender<TypieEvent>,
    current_selection: usize,
}

impl<'a> MainMenu<'a> {
    pub fn new(term_config: &'a TermConfig, tx: Sender<TypieEvent>) -> Self {
        Self {
            term_config,
            tx,
            current_selection: 0,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.current_selection == 0 {
                    return;
                }

                self.current_selection -= 1;
            }
            KeyCode::Down => {
                if self.current_selection == 3 {
                    return;
                }

                self.current_selection += 1;
            }
            KeyCode::Enter => match self.current_selection {
                0 => self
                    .tx
                    .send(TypieEvent::ChangeScreen(Screens::Test))
                    .unwrap(),
                3 => self.tx.send(TypieEvent::Exit).unwrap(),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn draw(&self, frame: &mut Frame<'_>) {
        let area = center(
            Constraint::Length(self.term_config.width),
            Constraint::Length(self.term_config.height),
            frame.area(),
        );

        let [logo, selections] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .areas(area);

        self.draw_logo(frame, logo);
        self.draw_selections(frame, selections);
    }

    fn draw_logo(&self, frame: &mut Frame<'_>, area: Rect) {
        let name = r#"
  _                      _
 / |_                   (_)
`| |-' _   __  _ .--.   __  .---.
 | |  [ \ [  ][ '/'`\ \[  |/ /__\\
 | |,  \ '/ /  | \__/ | | || \__.,
 \__/[\_:  /   | ;.__/ [___]'.__.'
      \__.'   [__|
        "#
        .trim();

        frame.render_widget(
            Text::from(name),
            center(
                Constraint::Length(
                    name.lines().map(|l| l.len()).max().unwrap() as u16,
                ),
                Constraint::Length(name.lines().count() as u16),
                area,
            ),
        );
    }

    fn draw_selections(&self, frame: &mut Frame<'_>, area: Rect) {
        let mut start_selection = Text::from("Start Test");
        if self.current_selection == 0 {
            start_selection = Text::from("> Start Test").fg(Color::Blue);
        }

        let mut help_seclection = Text::from("Help");
        if self.current_selection == 1 {
            help_seclection = Text::from("> Help").fg(Color::Blue);
        }

        let mut settings_seclection = Text::from("Settings");
        if self.current_selection == 2 {
            settings_seclection = Text::from("> Settings").fg(Color::Blue);
        }

        let mut exit_seclection = Text::from("Exit");
        if self.current_selection == 3 {
            exit_seclection = Text::from("> Exit").fg(Color::Blue);
        }

        frame.render_widget(
            start_selection,
            Rect::new(area.x, area.y, self.term_config.width, 1),
        );

        frame.render_widget(
            help_seclection,
            Rect::new(area.x, area.y + 2, self.term_config.width, 1),
        );

        frame.render_widget(
            settings_seclection,
            Rect::new(area.x, area.y + 4, self.term_config.width, 1),
        );

        frame.render_widget(
            exit_seclection,
            Rect::new(area.x, area.y + 6, self.term_config.width, 1),
        );
    }
}
