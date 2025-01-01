use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Constraint,
    prelude::CrosstermBackend,
    widgets::{Paragraph, Wrap},
    Terminal,
};
use std::{
    io::Stdout,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use crate::{
    config::{Config, TermConfig},
    ui::{Screens, Ui},
    utils::center,
};

pub enum TypieEvent {
    Tick,
    KeyPress(KeyCode),
    ChangeScreen(Screens),
    Exit,
}

pub struct Typie<'a> {
    config: &'a Config,
    term_config: TermConfig,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    channel: (Sender<TypieEvent>, Receiver<TypieEvent>),
}

impl<'a> Typie<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            term_config: TermConfig::new(),
            terminal: ratatui::init(),
            channel: channel(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let (tx, rx) = &self.channel;
        self.handle_input(tx.clone());
        self.tick(tx.clone());

        let mut ui = Ui::new(&self.term_config, tx.clone());

        loop {
            let event = rx.recv().map_err(|err| {
                format!("Failed to receive typie_event: {err}")
            })?;

            match event {
                TypieEvent::Tick => {
                    self.terminal.draw(|frame| {
                        if frame.area().width < self.term_config.width ||
                            frame.area().height < self.term_config.height {
                            let message = format!(
                                "Terminal is too small! Minimum width & height is: {}x{}",
                                self.term_config.width,
                                self.term_config.height
                            );
                            let message_len = message.len();

                            frame.render_widget(
                                Paragraph::new(message)
                                    .centered()
                                    .wrap(Wrap::default()),
                                center(Constraint::Length(message_len.to_owned() as u16), Constraint::Length(frame.area().height), frame.area())
                            );

                            return;
                        }

                        ui.draw(frame);
                    }).map_err(|err| format!("Failed to render frame: {err}"))?;
                }
                TypieEvent::KeyPress(key) => ui.handle_input(key),
                TypieEvent::ChangeScreen(screen) => ui.change_screen(screen),
                TypieEvent::Exit => break,
            }
        }

        ratatui::restore();

        Ok(())
    }

    fn handle_input(&self, tx: Sender<TypieEvent>) {
        thread::spawn(move || loop {
            match event::read().unwrap() {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    tx.send(TypieEvent::KeyPress(key.code)).unwrap();
                }
                _ => {}
            }
        });
    }

    fn tick(&self, tx: Sender<TypieEvent>) {
        thread::spawn(move || loop {
            tx.send(TypieEvent::Tick).unwrap();
            thread::sleep(Duration::from_millis(80));
        });
    }
}
