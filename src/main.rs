use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::time::{Duration, SystemTime};

use crossterm::{execute, terminal};
use exitfailure::ExitFailure;
use structopt::StructOpt;

mod key_handler;
mod notification;
mod sound;
mod view;

#[derive(StructOpt)]
struct Option {
    #[structopt(short = "w", long = "work-sec", default_value = "1500")]
    work_sec: u16,
    #[structopt(short = "s", long = "short-break-sec", default_value = "300")]
    short_break_sec: u16,
    #[structopt(short = "l", long = "long-break-sec", default_value = "1200")]
    long_break_sec: u16,
}

fn main() -> Result<(), ExitFailure> {
    // receive cli arguemnts
    let args = Option::from_args();

    // start key handler on another thread
    let receiver = key_handler::run();

    // start timer
    terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    let mut round: u64 = 1;
    loop {
        // work timer
        if start_timer(
            args.work_sec,
            round,
            &receiver,
            &mut stdout,
            view::flush_work_timer,
        )? {
            return Ok(());
        }

        notification::send("it's time to take a break \u{2615}")?;
        sound::play(sound::SoundFile::BELL)?;

        // break interval
        view::flush_break_interval(&mut stdout)?;
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }

        // break timer
        let break_sec = if round % 4 == 0 {
            args.long_break_sec
        } else {
            args.short_break_sec
        };
        if start_timer(
            break_sec,
            round,
            &receiver,
            &mut stdout,
            view::flush_break_timer,
        )? {
            return Ok(());
        }

        notification::send("it's time to work again!! \u{1F4AA}")?;
        sound::play(sound::SoundFile::BELL)?;

        // work interval
        view::flush_work_interval(&mut stdout)?;
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }

        round += 1;
    }
}

fn start_timer(
    remaining_sec: u16,
    current_round: u64,
    receiver: &Receiver<key_handler::KeyAction>,
    stdout: &mut Stdout,
    flush_fn: fn(s: &mut Stdout, t: u16, c: u64) -> Result<(), failure::Error>,
) -> Result<bool, failure::Error> {
    let mut quited = false;
    let mut paused = false;
    let mut remaining_sec = remaining_sec;
    let mut now = SystemTime::now();
    while remaining_sec != 0 {
        match handle_input_on_timer(receiver) {
            key_handler::KeyAction::Quit => {
                view::release_raw_mode(stdout)?;
                quited = true;
                break;
            }
            key_handler::KeyAction::Pause => paused = !paused,
            _ => (),
        }
        if !paused {
            flush_fn(stdout, remaining_sec, current_round)?;
            remaining_sec -= 1;
            // Handle case if computer is sleeping
            let elapsed = now.elapsed().unwrap().as_secs();
            // Possibly we can print how much time computer was sleeping
            if elapsed > 2 {
                remaining_sec = (remaining_sec as i16 + 1 - elapsed as i16).max(0) as u16;
            }
        }
        now = SystemTime::now();
        spin_sleep::sleep(Duration::from_secs(1));
    }
    Ok(quited)
}

fn handle_input_on_timer(receiver: &Receiver<key_handler::KeyAction>) -> key_handler::KeyAction {
    match receiver.try_recv() {
        Ok(key_handler::KeyAction::Quit) => key_handler::KeyAction::Quit,
        Ok(key_handler::KeyAction::Pause) => key_handler::KeyAction::Pause,
        _ => key_handler::KeyAction::None,
    }
}

fn handle_input_on_interval(
    stdout: &mut Stdout,
    receiver: &Receiver<key_handler::KeyAction>,
) -> Result<bool, failure::Error> {
    let mut quited = false;
    for received in receiver.iter() {
        match received {
            key_handler::KeyAction::Ok => break,
            key_handler::KeyAction::Quit => {
                view::release_raw_mode(stdout)?;
                quited = true;
                break;
            }
            _ => (),
        }
    }
    Ok(quited)
}
