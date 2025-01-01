use std::{io::Stdout, sync::mpsc::{channel, Receiver, Sender}, thread, time::Duration};
use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind}, prelude::CrosstermBackend, Terminal};

use crate::{config::Config, ui};

enum TypieEvent {
    Tick,
    KeyPress(KeyCode)
}

pub struct Typie<'a> {
    config: &'a Config,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    channel: (Sender<TypieEvent>, Receiver<TypieEvent>),
    exit: bool
}

impl<'a> Typie<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            terminal: ratatui::init(),
            channel: channel(),
            exit: false
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut letter_input: Option<KeyCode> = None;
        let (tx, rx) = &self.channel;

        self.handle_input(tx.clone());
        self.tick(tx.clone());

        loop {
            let event = rx.recv()
                .map_err(|err| format!("Failed to receive typie_event: {err}"))?;

            match event {
                TypieEvent::Tick => {
                    self.terminal.draw(|frame| {
                        ui::draw(frame, &letter_input);
                    }).map_err(|err| format!("Failed to render frame: {err}"))?;

                    letter_input = None;
                },
                TypieEvent::KeyPress(key) => {
                    if key == KeyCode::Esc {
                        break;
                    }

                    letter_input = Some(key);
                }
            }
        }

        ratatui::restore();

        Ok(())
    }

    fn handle_input(&self, tx: Sender<TypieEvent>) {
        thread::spawn(move || {
            loop {
                match event::read().unwrap() {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        tx.send(TypieEvent::KeyPress(key.code))
                            .unwrap();
                    },
                    _ => {}
                }
            }
        });
    }

    fn tick(&self, tx: Sender<TypieEvent>) {
        thread::spawn(move || {
            loop {
                tx.send(TypieEvent::Tick)
                    .unwrap();

                thread::sleep(Duration::from_millis(80));
            }
        });
    }
}
