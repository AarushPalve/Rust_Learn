use std::{fmt, hint::unreachable_unchecked};

use rand::seq::IndexedRandom;

#[derive(Clone, Copy)]
pub enum Face {
    U,
    D,
    R,
    L,
    B,
    F,
}

pub struct Move {
    face: Face,
    rotations: u8,
}

impl Face {
    const FACES: [Face; 6] = [Face::U, Face::D, Face::R, Face::L, Face::B, Face::F];
}

impl Move {
    pub fn new(face: Face, rotations: u8) -> Option<Self> {
        let valid = (1..=3).contains(&rotations);

        if valid {
            Some(Self { face, rotations })
        } else {
            None
        }
    }

    pub fn random_move() -> Self {
        let mut rng = rand::rng();
        let random_face = *Face::FACES.choose(&mut rng).unwrap();
        let random_rotation = rand::random_range(1..=3);
        Self::new(random_face, random_rotation).unwrap()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.rotations;
        let face = match self.face {
            Face::U => 'U',
            Face::D => 'D',
            Face::R => 'R',
            Face::L => 'L',
            Face::B => 'B',
            Face::F => 'F',
        };
        let _ = write!(f, "{face}");
        match r {
            1 => Ok(()),
            2 => write!(f, "2"),
            3 => write!(f, "'"),
            _ => unreachable!(),
        }
    }
}

pub fn random_scramble() -> String {
    let moves_count = rand::random_range(8..=12);
    let moves_vector = (0..moves_count)
        .map(|_| Move::random_move().to_string())
        .collect::<Vec<_>>();
    moves_vector.join(" ")
}
