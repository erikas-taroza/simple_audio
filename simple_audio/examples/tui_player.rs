// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

use std::{
    env,
    io::{self, stdout},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crossbeam::channel::unbounded;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use simple_audio::{types::*, Player};

struct App
{
    playback_state: PlaybackState,
    progress_state: ProgressState,
    looping: bool,
    volume: f32,
}

impl App
{
    fn new() -> Self
    {
        App {
            playback_state: PlaybackState::Stop,
            progress_state: ProgressState {
                position: Duration::ZERO,
                duration: Duration::ZERO,
            },
            looping: false,
            volume: 1.0,
        }
    }
}

fn main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");

    if args.len() <= 1 {
        panic!("Enter path or URL of file to play.");
    }

    let app = Arc::new(RwLock::new(App::new()));
    let thread_killer = unbounded::<bool>();
    let player = Player::new(thread_killer.1);
    player.open(args[1].clone(), true).unwrap();

    // Listen to events
    let receiver_app = app.clone();
    let receiver = player.event_receiver.clone();
    thread::spawn(move || {
        let app = receiver_app;
        let receiver = receiver;

        loop {
            match receiver.recv() {
                Ok(event) => {
                    let mut lock = app.write().unwrap();
                    match event {
                        PlayerEvent::PlaybackStarted(duration) => {
                            lock.progress_state = ProgressState {
                                position: Duration::ZERO,
                                duration,
                            }
                        }
                        PlayerEvent::Playback(playback) => {
                            lock.playback_state = playback;
                        }
                        PlayerEvent::Progress(progress) => {
                            lock.progress_state = progress;
                        }
                        PlayerEvent::Error(err) => panic!("Unexpected player error: {err}"),
                    }
                }
                Err(err) => panic!("Unexpected error: {err}"),
            }
        }
    });

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| ui(f, &app.read().unwrap()))?;
        should_quit = handle_events(&player, &mut app.write().unwrap())?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(frame: &mut Frame, app: &App)
{
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());

    frame.render_widget(
        Block::new()
            .title("Simple Audio")
            .title_alignment(Alignment::Center),
        main_layout[0],
    );

    let info_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ],
    )
    .split(main_layout[1]);

    frame.render_widget(
        Text::from(format!("Playback State: {:?}", app.playback_state)),
        info_layout[0],
    );
    frame.render_widget(
        Text::from(format!("Looping: {}", app.looping)),
        info_layout[1],
    );
    frame.render_widget(
        Text::from(format!("Volume: {}%", (app.volume * 100.0).ceil())),
        info_layout[2],
    );

    let pos = app.progress_state.position.as_secs();
    let dur = app.progress_state.duration.as_secs();
    let progress_bar = Gauge::default()
        .label(format!(
            "{}/{}",
            seconds_to_string(pos),
            seconds_to_string(dur)
        ))
        .ratio(if dur == 0 {
            0.0
        }
        else {
            pos as f64 / dur as f64
        });

    frame.render_widget(progress_bar, info_layout[3]);

    let controls_layout =
        Layout::new(Direction::Horizontal, [Constraint::Ratio(1, 5); 5]).split(main_layout[2]);
    frame.render_widget(Block::new().bg(Color::Blue), main_layout[2]);
    frame.render_widget(Text::from("[P] Play/Pause"), controls_layout[0]);
    frame.render_widget(Text::from("[S] Stop"), controls_layout[1]);
    frame.render_widget(Text::from("[L] Loop"), controls_layout[2]);
    frame.render_widget(Text::from("[Up] Volume Up"), controls_layout[3]);
    frame.render_widget(Text::from("[Down] Volume Down"), controls_layout[4]);
}

fn handle_events(player: &Player, app: &mut App) -> io::Result<bool>
{
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('s') => player.stop(),
                    KeyCode::Char('p') => match player.playback_state() {
                        PlaybackState::Play => player.pause(),
                        PlaybackState::Pause => player.play(),
                        _ => return Ok(false),
                    },
                    KeyCode::Char('l') => {
                        player.loop_playback(!player.is_looping());
                        app.looping = player.is_looping();
                    }
                    KeyCode::Up => {
                        player.set_volume((player.volume() + 0.1).clamp(0.0, 1.0));
                        app.volume = player.volume();
                    }
                    KeyCode::Down => {
                        player.set_volume((player.volume() - 0.1).clamp(0.0, 1.0));
                        app.volume = player.volume();
                    }
                    _ => return Ok(false),
                }
            }
        }
    }
    Ok(false)
}

fn seconds_to_string(seconds: u64) -> String
{
    let m = seconds / 60;
    let s = seconds % 60;
    format!("{:02}:{:02}", m, s)
}
