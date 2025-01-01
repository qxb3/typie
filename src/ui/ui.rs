use ratatui::{
    crossterm::event::KeyCode, layout::{
        Constraint,
        Layout,
        Rect
    }, style::{Color, Stylize}, widgets::{
        Block,
        Borders,
        Paragraph,
        Wrap
    }, Frame
};

use crate::utils::center;

const WIDTH: u16 = 60;
const HEIGHT: u16 = 24;

pub fn draw(frame: &mut Frame<'_>, key: &Option<KeyCode>) {
    if frame.area().width < WIDTH ||
        frame.area().height < HEIGHT {
        let message = format!("Terminal is too small! Minimum width & height is: {WIDTH}x{HEIGHT}");
        let message_len = message.len();

        frame.render_widget(
            Paragraph::new(message)
                .centered()
                .wrap(Wrap::default()),
            center(Constraint::Length(message_len.to_owned() as u16), Constraint::Length(frame.area().height), frame.area())
        );

        return;
    }

    let area = center(
        Constraint::Length(WIDTH),
        Constraint::Length(HEIGHT),
        frame.area()
    );

    // frame.render_widget(
    //     Text::from(format!("{:?}", key)),
    //     area,
    // );

    let [top, bottom] = Layout::vertical([
        Constraint::Percentage(50),
        Constraint::Percentage(50)
    ]).areas(area);

    for (i, letter) in ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"].iter().enumerate() {
        if let Some(KeyCode::Char(char)) = key {
            if char.to_uppercase().to_string() == letter.to_string() {
                frame.render_widget(
                    Paragraph::new(letter.to_string())
                        .bold()
                        .centered()
                        .block(
                            Block::new()
                                .fg(Color::Blue)
                                .borders(Borders::ALL)
                        ),
                    Rect::new(bottom.x + i as u16 * 6, bottom.y, 5, 3)
                );

                continue;
            }
        }

        frame.render_widget(
            Paragraph::new(letter.to_string())
                .bold()
                .centered()
                .block(Block::new().borders(Borders::ALL)),
            Rect::new(bottom.x + i as u16 * 6, bottom.y, 5, 3)
        );
    }

    for (i, letter) in ["A", "S", "D", "F", "G", "H", "J", "K", "L"].iter().enumerate() {
        if let Some(KeyCode::Char(char)) = key {
            if char.to_uppercase().to_string() == letter.to_string() {
                frame.render_widget(
                    Paragraph::new(letter.to_string())
                        .bold()
                        .centered()
                        .block(
                            Block::new()
                                .fg(Color::Blue)
                                .borders(Borders::ALL)
                        ),
                    Rect::new(bottom.x + 3 + i as u16 * 6, bottom.y + 3, 5, 3)
                );

                continue;
            }
        }

        frame.render_widget(
            Paragraph::new(letter.to_string())
                .bold()
                .centered()
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                ),
            Rect::new(bottom.x + 3 + i as u16 * 6, bottom.y + 3, 5, 3)
        );
    }

    for (i, letter) in ["Z", "X", "C", "V", "B", "N", "M"].iter().enumerate() {
        if let Some(KeyCode::Char(char)) = key {
            if char.to_uppercase().to_string() == letter.to_string() {
                frame.render_widget(
                    Paragraph::new(letter.to_string())
                        .bold()
                        .centered()
                        .block(
                            Block::new()
                                .fg(Color::Blue)
                                .borders(Borders::ALL)
                        ),
                    Rect::new(bottom.x + 9 + i as u16 * 6, bottom.y + 6, 5, 3)
                );

                continue;
            }
        }

        frame.render_widget(
            Paragraph::new(letter.to_string())
                .bold()
                .centered()
                .block(Block::new().borders(Borders::ALL)),
            Rect::new(bottom.x + 9 + i as u16 * 6, bottom.y + 6, 5, 3)
        );
    }

    frame.render_widget(
        Paragraph::new("")
            .bold()
            .centered()
            .block(Block::new().borders(Borders::ALL)),
        Rect::new(bottom.x + 11, bottom.y + 9, 36, 3)
    );
}
