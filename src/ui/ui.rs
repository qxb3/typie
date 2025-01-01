use std::sync::mpsc::Sender;

use crate::{config::TermConfig, typie::TypieEvent};
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
    pub fn new(term_config: &'a TermConfig, tx: Sender<TypieEvent>) -> Self {
        Self {
            term_config,
            current_screen: Screens::MainMenu,

            main_menu: MainMenu::new(term_config, tx.clone()),
            test: Test::new(term_config, tx.clone()),
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
            Screens::Test => self.test.handle_input(key),
            _ => {}
        }
    }

    pub fn change_screen(&mut self, screen: Screens) {
        self.current_screen = screen;
    }
}
