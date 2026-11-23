mod app;
mod ui;
mod tabs;
mod commands;

use app::{ActiveTab, App};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
// use ratatui::DefaultTerminal;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut app = App::new();
        loop {
            terminal.draw(|frame| ui::draw(frame, &app))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::F(1) => app.active_tab = ActiveTab::Console,
                    KeyCode::F(2) => app.active_tab = ActiveTab::Stats,
                    KeyCode::F(3) => app.active_tab = ActiveTab::Shop,
                    KeyCode::F(4) => app.active_tab = ActiveTab::Settings,
                    KeyCode::Char(c) if let ActiveTab::Console = app.active_tab => {
                        app.console.push_input(c);
                    },
                    KeyCode::Backspace if let ActiveTab::Console = app.active_tab => {
                        app.console.pop_input();
                    },
                    KeyCode::Enter if let ActiveTab::Console = app.active_tab => {
                        let cmd = app.console.enter();

                        if cmd.trim() == "exit" {
                            app.should_quit = true;
                        } else if cmd.trim() == "clear" {
                            app.console.clear();
                        } else {
                            let response = commands::handle_command(&cmd);

                            app.console.print(response);
                        }
                    }
                    _ => {}
                }
                if app.should_quit {
                    break Ok(());
                }
            }
        }
    })
}