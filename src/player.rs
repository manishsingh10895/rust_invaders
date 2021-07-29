use std::time::Duration;

use crate::{NUM_COLS, NUM_ROWS, frame::Drawable, invaders::Invaders, shot::Shot};

const MAX_SHOTS: usize = 4;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < MAX_SHOTS {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }

        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool{
        let mut hit_something = false;

        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                    shot.explode();
                }
            }
        }

        hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "A";

        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}