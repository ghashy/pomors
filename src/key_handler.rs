use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use crossterm::event::{read, KeyCode, KeyEvent, KeyEventKind};

pub enum KeyAction {
    Quit,
    Pause,
    Ok,
    None,
}

pub fn run() -> Receiver<KeyAction> {
    let (sender, receiver) = mpsc::channel::<KeyAction>();
    thread::spawn(move || loop {
        let c = read();
        match c.as_ref().unwrap() {
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => {
                sender.send(KeyAction::Ok).unwrap();
            }
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                kind: KeyEventKind::Press,
                ..
            }) => {
                sender.send(KeyAction::Pause).unwrap();
            }
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                sender.send(KeyAction::Quit).unwrap();
                break;
            }
            _ => {}
        }
    });
    receiver
}
