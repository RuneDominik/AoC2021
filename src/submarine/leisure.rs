#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

struct DetDice {
    state: u16,
}

impl DetDice {
    pub fn new() -> DetDice {
        DetDice { state: 0 }
    }
}

trait Roll {
    fn roll(&mut self) -> u16;
}

impl Roll for DetDice {
    fn roll(&mut self) -> u16 {
        let mut result = 0;
        for _i in 0..3 {
            self.state += 1;
            result += &self.state;
        }
        return result;
    }
}

struct Game {
    dice: Box<dyn Roll>,
    pos1: u16,
    pos2: u16,
    score1: u64,
    score2: u64,
    rolls: u64,
}

impl Game {
    pub fn new(dice: Box<dyn Roll>, pos1: u16, pos2: u16) -> Game {
        Game {
            dice: dice,
            pos1: pos1,
            pos2: pos2,
            score1: 0,
            score2: 0,
            rolls: 0,
        }
    }

    pub fn play(mut self) -> u64 {
        while self.score1 < 1000 && self.score2 < 1000 {
            let travel_dist = self.dice.roll();
            self.rolls += 3;
            self.pos1 += travel_dist % 10;
            self.pos1 %= 10;

            if self.pos1 == 0 {
                self.pos1 = 10;
            }

            self.score1 += self.pos1 as u64;

            if self.score1 >= 1000 {
                break;
            }

            let travel_dist = self.dice.roll();
            self.rolls += 3;
            self.pos2 += travel_dist % 10;
            self.pos2 %= 10;

            if self.pos2 == 0 {
                self.pos2 = 10;
            }
            self.score2 += self.pos2 as u64;
        }

        if self.score1 >= 1000 {
            return self.score2 * self.rolls;
        } else {
            return self.score1 * self.rolls;
        }
    }
}

pub fn deterministic_dirac(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let lines = br.lines().map(|x| x.unwrap());

    let deterministic_dice = Box::new(DetDice::new());

    let mut start_pos = Vec::<u16>::new();

    for line in lines {
        let split: Vec<&str> = line.split(": ").collect();
        start_pos.push(split[1].parse::<u16>().unwrap());
    }

    let game = Game::new(deterministic_dice, start_pos[0], start_pos[1]);
    let score: u64 = game.play();

    return Ok(score);
}
