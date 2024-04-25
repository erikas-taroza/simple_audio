use std::{
    env,
    io::{self, stdout},
};

use chrono::Duration;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use simple_audio::{types::*, Player};

fn main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");

    if args.len() <= 1 {
        panic!("Enter path or URL of file to play.");
    }

    let player = Player::new();
    player.open(args[1].clone(), true).unwrap();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| ui(f, &player))?;
        should_quit = handle_events(&player)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(frame: &mut Frame, player: &Player)
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
        Text::from(format!("Playback State: {:?}", player.playback_state())),
        info_layout[0],
    );
    frame.render_widget(
        Text::from(format!("Looping: {}", player.is_looping())),
        info_layout[1],
    );
    frame.render_widget(
        Text::from(format!("Volume: {}%", (player.volume() * 100.0).ceil())),
        info_layout[2],
    );

    let pos = player.progress().position.num_seconds();
    let dur = player.progress().duration.num_seconds();
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

fn handle_events(player: &Player) -> io::Result<bool>
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
                    KeyCode::Char('l') => player.loop_playback(!player.is_looping()),
                    KeyCode::Up => player.set_volume((player.volume() + 0.1).clamp(0.0, 1.0)),
                    KeyCode::Down => player.set_volume((player.volume() - 0.1).clamp(0.0, 1.0)),
                    _ => return Ok(false),
                }
            }
        }
    }
    Ok(false)
}

fn seconds_to_string(seconds: i64) -> String
{
    let m = seconds / 60;
    let s = seconds % 60;
    format!("{:02}:{:02}", m, s)
}
