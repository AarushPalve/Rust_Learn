use std::{
    sync::mpsc::{RecvTimeoutError, channel},
    time::Duration,
};

use getch::Getch;

use crate::{
    scramble_maker::{Face, Move, random_scramble},
    timer::Timer,
};

mod scramble_maker;
mod timer;

fn main() {
    let crazy_move = Move::new(Face::U, 2).unwrap();
    crazy_move.to_string();
    let mut timer = Timer::default();

    let (sender, reciever) = channel();

    std::thread::spawn(move || {
        let getch = Getch::new();
        loop {
            let c = getch.getch().unwrap();
            if c == 27 {
                drop(sender);
                break;
            }
            sender.send(c).unwrap();
        }
    });
    println!("{}", random_scramble());
    loop {
        match reciever.recv_timeout(Duration::from_millis(50)) {
            Ok(c) => {
                timer.key_pressed();
            }
            Err(RecvTimeoutError::Disconnected) => break,
            Err(RecvTimeoutError::Timeout) => {
                timer.tick();
            }
        };
    }
    println!("bye bye");
}
