extern crate failure;
extern crate log;
extern crate stderrlog;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

use util::event::{Event, Events};

struct App {
    size: Rect,
}

impl App {
    fn new() -> App {
        App {
            size: Rect::default(),
        }
    }
}

fn main() -> Result<(), failure::Error> {
    stderrlog::new().verbosity(4).init()?;

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        let size = terminal.size()?;
        if size != app.size {
            terminal.resize(size)?;
            app.size = size;
        }

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                        .as_ref(),
                ).split(app.size);

            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .render(&mut f, chunks[0]);
            Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .render(&mut f, chunks[2]);
        })?;

        match events.next()? {
            Event::Input(input) => if let Key::Char('q') = input {
                break;
            },
            _ => {}
        }
    }

    Ok(())
}
