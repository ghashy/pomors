use std::io::Write;

use crossterm::{
    cursor::{MoveTo, Show},
    execute,
    style::{ContentStyle, StyledContent},
    terminal::Clear,
};
use failure::ResultExt;

pub fn flush_work_timer(
    stdout: &mut impl Write,
    remaining_sec: u16,
    current_round: u64,
) -> Result<(), failure::Error> {
    execute!(
        stdout,
        MoveTo(2, 1),
        Clear(crossterm::terminal::ClearType::All),
        crossterm::style::PrintStyledContent(StyledContent::new(
            ContentStyle {
                foreground_color: Some(crossterm::style::Color::Red),
                ..Default::default()
            },
            format!(
                "\u{1F345} {} (Round {current_round})",
                convert_to_min(remaining_sec)
            ),
        )),
        MoveTo(2, 2),
        crossterm::style::Print(format!("[Q]: quit, [Space]: pause/resume")),
    )
    .context("failed to show work timer")?;
    stdout.flush().context("failed to flush work timer")?;
    Ok(())
}

pub fn flush_break_timer(
    stdout: &mut impl Write,
    remaining_sec: u16,
    current_round: u64,
) -> Result<(), failure::Error> {
    execute!(
        stdout,
        MoveTo(2, 1),
        Clear(crossterm::terminal::ClearType::All),
        crossterm::style::PrintStyledContent(StyledContent::new(
            ContentStyle {
                foreground_color: Some(crossterm::style::Color::Green),
                ..Default::default()
            },
            format!(
                "\u{2615} {} (Round {current_round})",
                convert_to_min(remaining_sec)
            ),
        )),
        MoveTo(2, 2),
        crossterm::style::Print(format!("[Q]: quit, [Space]: pause/resume")),
    )
    .context("failed to show break timer")?;
    stdout.flush().context("failed to flush break timer")?;
    Ok(())
}

pub fn flush_break_interval(stdout: &mut impl Write) -> Result<(), failure::Error> {
    execute!(
        stdout,
        MoveTo(2, 1),
        Clear(crossterm::terminal::ClearType::All),
        crossterm::style::PrintStyledContent(StyledContent::new(
            ContentStyle {
                foreground_color: Some(crossterm::style::Color::Green),
                ..Default::default()
            },
            format!("\u{1F389} press Enter to take a break",),
        )),
        MoveTo(2, 2),
        crossterm::style::Print(format!("[Q]: quit, [Enter]: start")),
    )
    .context("failed to show break interval")?;
    stdout.flush().context("failed to flush break interval")?;
    Ok(())
}

pub fn flush_work_interval(stdout: &mut impl Write) -> Result<(), failure::Error> {
    execute!(
        stdout,
        MoveTo(2, 1),
        Clear(crossterm::terminal::ClearType::All),
        crossterm::style::PrintStyledContent(StyledContent::new(
            ContentStyle {
                foreground_color: Some(crossterm::style::Color::Red),
                ..Default::default()
            },
            format!("\u{1F389} press Enter to work!!",),
        )),
        MoveTo(2, 2),
        crossterm::style::Print(format!("[Q]: quit, [Enter]: start")),
    )
    .context("failed to show work interval")?;
    stdout.flush().context("failed to flush work interval")?;
    Ok(())
}

pub fn release_raw_mode(stdout: &mut impl Write) -> Result<(), failure::Error> {
    execute!(stdout, MoveTo(1, 1), Show).context("failed to release raw mode")?;
    Ok(())
}

fn convert_to_min(duration: u16) -> String {
    let min = duration / 60;
    let sec = duration % 60;
    format!("{:02}:{:02}", min, sec)
}

#[cfg(test)]
mod tests {
    use crate::view::*;

    #[test]
    fn flush_work_timer_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_work_timer(&mut buf, 4, 1);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("00:04 (Round 1)"));
        assert!(actual_view.contains("[Q]: quit, [Space]: pause/resume"));
    }

    #[test]
    fn flush_break_timer_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_break_timer(&mut buf, 604, 2);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("10:04 (Round 2)"));
        assert!(actual_view.contains("[Q]: quit, [Space]: pause/resume"));
    }

    #[test]
    fn flush_break_interval_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_break_interval(&mut buf);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("press Enter to take a break"));
        assert!(actual_view.contains("[Q]: quit, [Enter]: start"));
    }

    #[test]
    fn flush_work_interval_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_work_interval(&mut buf);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("press Enter to work!!"));
        assert!(actual_view.contains("[Q]: quit, [Enter]: start"));
    }
}
