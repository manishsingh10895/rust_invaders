use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use invadors::frame::{new_frame, Drawable};
use invadors::invaders::Invaders;
use invadors::player::Player;
use invadors::{frame, render};
use rusty_audio::Audio;
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::{io, vec};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./explode.wav");
    audio.add("lose", "./lose.wav");
    audio.add("move", "./move.wav");
    audio.add("pew", "./pew.wav");
    audio.add("startup", "./startup.wav");
    audio.add("win", "./win.wav");

    audio.play("startup");

    let mut stdout = io::stdout();

    let _ = terminal::enable_raw_mode();
    stdout.execute(EnterAlternateScreen)?;

    let _ = stdout.execute(Hide);

    //Make a render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();

        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut instant = Instant::now();
    //Init Drawables
    let mut player = Player::new();
    let mut invaders = Invaders::new();

    //Game loop
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        //Per-fram init
        let mut curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('a') => player.move_left(),
                    KeyCode::Left => player.move_left(),
                    KeyCode::Char('d') => player.move_right(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");

                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        //Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }

        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        //Draw & render
        let drawbles: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawbles {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    
        //WIN or LOSE
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }

        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
