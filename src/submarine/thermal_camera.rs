#![allow(dead_code)]

use itertools::Itertools;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

#[derive(Clone, Hash, Copy)]
struct Point {
    x: u64,
    y: u64,
}
impl Point {
    pub fn new(point: (u64, u64)) -> Point {
        let x = point.0;
        let y = point.1;
        Point { x: x, y: y }
    }
    pub fn change_x(x: &mut u64, x_val: u64) {
        *x = x_val;
    }
    pub fn change_y(y: &mut u64, y_val: u64) {
        *y = y_val;
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return (self.x == other.x) && (self.y == other.y);
    }
}
impl Eq for Point {}

#[derive(Clone, Hash)]
struct Map {
    map: Vec<Point>,
}
impl Map {
    pub fn new(points: Vec<(u64, u64)>) -> Map {
        let mut map = Vec::<Point>::new();
        for point in points {
            map.push(Point::new(point));
        }
        Map { map: map }
    }

    fn fold_x(map: &mut Vec<Point>, x_val: u64) {
        for point in map {
            let delta_x: i64 = point.x as i64 - x_val as i64;
            if delta_x < 0 {
                continue;
            } else {
                let new_x: u64 = (x_val as i64 - delta_x) as u64;
                Point::change_x(&mut point.x, new_x);
            }
        }
    }

    fn fold_y(map: &mut Vec<Point>, y_val: u64) {
        for point in map {
            let delta_y: i64 = point.y as i64 - y_val as i64;
            //println!("{}", &delta_y);
            if delta_y < 0 {
                continue;
            } else {
                let new_y: u64 = (y_val as i64 - delta_y) as u64;
                //println!("{}", &new_y);
                Point::change_y(&mut point.y, new_y);
            }
        }
    }

    pub fn fold(&mut self, fold_statement: (&str, u64)) {
        if fold_statement.0 == "x" {
            Map::fold_x(&mut self.map, fold_statement.1);
        } else if fold_statement.0 == "y" {
            Map::fold_y(&mut self.map, fold_statement.1);
        }
    }

    pub fn count(self) -> usize {
        return self.map.into_iter().unique().collect::<Vec<Point>>().len();
    }
}

pub fn get_first_fold(path: impl AsRef<Path>) -> Result<usize, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let lines = br.lines().map(|x| x.unwrap());

    let mut points = Vec::<(u64, u64)>::new();
    let mut folds = Vec::<(&str, u64)>::new();

    for line in lines {
        if line.contains("x") {
            let substring = line.replace("fold along x=", "");
            folds.push(("x", substring.parse::<u64>().unwrap()));
        } else if line.contains("y") {
            let substring = line.replace("fold along y=", "");
            folds.push(("y", substring.parse::<u64>().unwrap()));
        } else if line.is_empty() {
            continue;
        } else {
            let point_string: Vec<&str> = line.split(',').collect();
            points.push((
                point_string[0].parse::<u64>().unwrap(),
                point_string[1].parse::<u64>().unwrap(),
            ));
        }
    }

    let mut map = Map::new(points);
    map.fold(folds[0]);

    return Ok(map.count());
}

pub fn get_code(path: impl AsRef<Path>) -> Result<usize, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let lines = br.lines().map(|x| x.unwrap());

    let mut points = Vec::<(u64, u64)>::new();
    let mut folds = Vec::<(&str, u64)>::new();

    for line in lines {
        if line.contains("x") {
            let substring = line.replace("fold along x=", "");
            folds.push(("x", substring.parse::<u64>().unwrap()));
        } else if line.contains("y") {
            let substring = line.replace("fold along y=", "");
            folds.push(("y", substring.parse::<u64>().unwrap()));
        } else if line.is_empty() {
            continue;
        } else {
            let point_string: Vec<&str> = line.split(',').collect();
            points.push((
                point_string[0].parse::<u64>().unwrap(),
                point_string[1].parse::<u64>().unwrap(),
            ));
        }
    }

    let mut map = Map::new(points);

    for fold_instr in folds {
        map.fold(fold_instr);
    }

    let mut f = File::create("data/day13_data/result_part_2.txt").expect("Unable to create file");
    for &point in &map.map {
        write!(f, "{},{}\n", point.x, point.y).expect("Unable to write data");
    }

    return Ok(map.count());
}
