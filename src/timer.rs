use std::{io::Write, time::Instant};

use crate::scramble_maker::random_scramble;

#[derive(Default)]
pub enum TimerState {
    #[default]
    Idle,
    CountingDown(Instant),
    Solving(Instant),
}

#[derive(Default)]
pub struct Timer {
    timer_state: TimerState,
}

impl Timer {
    pub fn key_pressed(&mut self) {
        match self.timer_state {
            TimerState::Idle => {
                self.timer_state = TimerState::CountingDown(Instant::now());
            }
            TimerState::CountingDown(_) => {
                self.timer_state = TimerState::Solving(Instant::now());
            }
            TimerState::Solving(_) => {
                println!(
                    "You solved it in: {:.2} seconds :D",
                    self.get_timer_reading()
                );
                println!("{}", random_scramble());
                self.timer_state = TimerState::Idle;
            }
        }
    }

    pub fn get_timer_reading(&self) -> f64 {
        match self.timer_state {
            TimerState::Idle => 15.0,
            TimerState::CountingDown(pressed_space_time) => {
                15.0 - pressed_space_time.elapsed().as_secs_f64()
            }
            TimerState::Solving(pressed_solve_time) => pressed_solve_time.elapsed().as_secs_f64(),
        }
    }

    pub fn tick(&self) {
        print!("{:.2}     \r", self.get_timer_reading());
        std::io::stdout().flush().unwrap();
    }
}