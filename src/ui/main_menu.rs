use ratatui::{
    crossterm::style::Color,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Text,
    Frame,
};

use crate::{config::TermConfig, utils::center};

#[derive(PartialEq)]
enum Selections {
    Start,
    Help,
    Settings,
    Exit,
}

pub struct MainMenu<'a> {
    term_config: &'a TermConfig,
    current_selection: Selections,
}

impl<'a> MainMenu<'a> {
    pub fn new(term_config: &'a TermConfig) -> Self {
        Self {
            term_config,
            current_selection: Selections::Start,
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
        if self.current_selection == Selections::Start {
            start_selection = Text::from("> Start Test").fg(Color::Blue);
        }

        let mut help_seclection = Text::from("Help");
        if self.current_selection == Selections::Help {
            help_seclection = Text::from("> Help").fg(Color::Blue);
        }

        let mut settings_seclection = Text::from("Settings");
        if self.current_selection == Selections::Settings {
            settings_seclection = Text::from("> Settings").fg(Color::Blue);
        }

        let mut exit_seclection = Text::from("Exit");
        if self.current_selection == Selections::Exit {
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
