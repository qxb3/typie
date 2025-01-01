use crate::config::TermConfig;
use ratatui::{crossterm::event::KeyCode, Frame};

use super::{main_menu::MainMenu, test::Test};

#[derive(Debug, PartialEq)]
pub enum Screens {
    MainMenu,
    Test,
    Help,
    Settings,
}

pub struct Ui<'a> {
    term_config: &'a TermConfig,
    pub current_screen: Screens,
    pub main_menu: MainMenu<'a>,
    pub test: Test<'a>,
}

impl<'a> Ui<'a> {
    pub fn new(term_config: &'a TermConfig) -> Self {
        Self {
            term_config,
            current_screen: Screens::MainMenu,

            main_menu: MainMenu::new(term_config),
            test: Test::new(term_config),
        }
    }

    pub fn draw(&self, frame: &mut Frame<'_>) {
        match self.current_screen {
            Screens::MainMenu => self.main_menu.draw(frame),
            Screens::Test => self.test.draw(frame),
            Screens::Help => {}
            Screens::Settings => {}
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match self.current_screen {
            Screens::MainMenu => self.main_menu.handle_input(key),
            _ => {}
        }
    }

    // pub fn set_screen(&mut self, screen: Screens) {
    //     self.current_screen = screen;
    // }
}
