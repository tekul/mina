use crate::events::{Event, Events};
use clap::Clap;
use std::{error::Error, io};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::Terminal;

mod api;
mod app;
mod db;
mod events;
mod ui;
mod widgets;

#[derive(Clap)]
#[clap(version = "0.1")]
struct Opts {
    dlna_address: String,
    naim_address: String,
}

fn check_http_prefix(addr: String) -> String {
    if addr.starts_with("http://") {
        addr
    } else {
        format!("http://{}", addr)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    //let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let src_addr = check_http_prefix(opts.dlna_address);
    let dest_addr = check_http_prefix(opts.naim_address);

    let events = Events::new();
    let tracks = db::read_tracks()?;
    let naim_api = api::Api::new(dest_addr.as_str(), src_addr.as_str());
    let mut app = app::App::new(naim_api, &tracks);

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Up => {
                    app.on_up();
                }
                Key::Down => {
                    app.on_down();
                }
                Key::PageDown => {
                    app.on_page_down();
                }
                Key::PageUp => {
                    app.on_page_up();
                }
                Key::Backspace => {
                    app.on_backspace();
                }
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }
    Ok(())
}
