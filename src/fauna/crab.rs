#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

pub fn cost_lin(x: i64, pos: &Vec<i64>) -> i64 {
    return pos.into_iter().map(|elem| i64::abs(elem - x)).sum::<i64>();
}

pub fn cost_quad(x: i64, pos: &Vec<i64>) -> i64 {
    let distances = pos.into_iter().map(|elem| i64::abs(elem - x));
    return distances
        .map(|elem| (elem.pow(2) + elem) as f64 / 2.0)
        .sum::<f64>() as i64;
}

pub fn get_lin_consumption(path: impl AsRef<Path>) -> Result<i64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let pos: Vec<i64> = br
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut costs = Vec::<i64>::new();

    let min = pos.iter().min().unwrap();
    let max = pos.iter().max().unwrap();

    for x in *min..*max {
        costs.push(cost_lin(x, &pos));
    }

    return Ok(*costs.iter().min().unwrap());
}

pub fn get_quad_consumption(path: impl AsRef<Path>) -> Result<i64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let pos: Vec<i64> = br
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut costs = Vec::<i64>::new();

    let min = pos.iter().min().unwrap();
    let max = pos.iter().max().unwrap();

    for x in *min..*max {
        costs.push(cost_quad(x, &pos));
    }

    return Ok(*costs.iter().min().unwrap());
}
