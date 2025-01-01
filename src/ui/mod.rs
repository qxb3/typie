mod ui;
mod test;

use ratatui::{crossterm::event::KeyCode, Frame};

pub fn draw(frame: &mut Frame<'_>, key: &Option<KeyCode>) {
    ui::draw(frame, key);
}
