#![allow(dead_code)]

use itertools::{rev, sorted};
use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
    path::Path,
};

pub fn get_risk_level(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let mut ground = Vec::new();
    for line in br.lines() {
        ground.push(
            line.unwrap()
                .split("")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.parse::<u8>().unwrap_or(9))
                .collect::<Vec<_>>(),
        );
    }

    let x_len: usize = ground[0].len();

    ground.push(vec![9; x_len]);
    ground.push(vec![9; x_len]);
    ground.rotate_right(1);
    let y_len: usize = ground.len();

    let mut risk_level: u64 = 0;
    for y_pos in 1..y_len - 1 {
        for x_pos in 1..x_len {
            let current: u8 = ground[y_pos][x_pos];
            if (current < ground[y_pos + 1][x_pos])
                && (current < ground[y_pos - 1][x_pos])
                && (current < ground[y_pos][x_pos - 1])
                && (current < ground[y_pos][x_pos + 1])
            {
                risk_level += 1 + current as u64;
            }
        }
    }
    return Ok(risk_level);
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    value: u8,
    x: usize,
    y: usize,
}

impl Point {
    /// Creates a new Point.
    fn new(value: u8, x: usize, y: usize) -> Point {
        Point {
            value: value,
            x: x,
            y: y,
        }
    }
}

pub fn follow_gradient(
    ground: &Vec<Vec<u8>>,
    x_pos: usize,
    y_pos: usize,
) -> Result<Vec<usize>, Error> {
    let current: u8 = ground[y_pos][x_pos];

    if current == 9 {
        return Err(Error::new(ErrorKind::Other, "oh no, a 9!"));
    }

    let neighbors = HashMap::from([
        (
            "upper",
            Point::new(ground[y_pos + 1][x_pos], x_pos, y_pos + 1),
        ),
        (
            "lower",
            Point::new(ground[y_pos - 1][x_pos], x_pos, y_pos - 1),
        ),
        (
            "right",
            Point::new(ground[y_pos][x_pos + 1], x_pos + 1, y_pos),
        ),
        (
            "left",
            Point::new(ground[y_pos][x_pos - 1], x_pos - 1, y_pos),
        ),
    ]);

    if (current < neighbors.get("upper").unwrap().value)
        && (current < neighbors.get("lower").unwrap().value)
        && (current < neighbors.get("left").unwrap().value)
        && (current < neighbors.get("right").unwrap().value)
    {
        return Ok(vec![x_pos, y_pos]);
    } else {
        let lowest_point = neighbors
            .iter()
            .min_by(|a, b| a.1.value.cmp(&b.1.value))
            .unwrap();
        return follow_gradient(ground, lowest_point.1.x, lowest_point.1.y);
    }
}

pub fn get_basin_size(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let mut ground = Vec::new();
    for line in br.lines() {
        ground.push(
            line.unwrap()
                .split("")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.parse::<u8>().unwrap_or(9))
                .collect::<Vec<_>>(),
        );
    }

    let x_len: usize = ground[0].len();

    ground.push(vec![9; x_len]);
    ground.push(vec![9; x_len]);
    ground.rotate_right(1);
    let y_len: usize = ground.len();

    let mut target = Vec::new();

    for y_pos in 1..y_len - 1 {
        for x_pos in 1..x_len - 1 {
            let res = follow_gradient(&ground, x_pos, y_pos);
            if res.is_ok() {
                target.push(res.unwrap());
            }
        }
    }

    let mut basins = HashMap::new();

    for point in target {
        let counter = basins.entry(point).or_insert(0);
        *counter += 1;
    }

    let mut values = rev(sorted(basins.iter().map(|x| *x.1)));

    let score: u64 = values.next().unwrap() as u64
        * values.next().unwrap() as u64
        * values.next().unwrap() as u64;

    return Ok(score);
}
