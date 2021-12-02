#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};
pub fn get_course_auto(path: impl AsRef<Path>) -> Result<i64, Error>{
    let file = File::open(path)?;
    
    let br = BufReader::new(file);

    let course = br.lines();
    //               .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    //               .collect();

    let mut forward = 0;
    let mut depth = 0;

    //let mut direction = String::new();
    //let mut amount = 0;

    for line in course {
        let unwr_line = line.unwrap();
        let mut split_line = unwr_line.split(' ');

        let direction = split_line.nth(0).unwrap();
        let amount = split_line.nth(0).unwrap().parse::<i64>().unwrap();

        if direction == "forward" {
            forward += amount;
        }
        else if direction == "up" {
            depth -= amount;
        }
        else if direction == "down" {
            depth += amount;
        }
    }

    return Ok(forward*depth);
}
pub fn get_course_manual(path: impl AsRef<Path>) -> Result<i64, Error>{
    let file = File::open(path)?;
    
    let br = BufReader::new(file);

    let course = br.lines();
    //               .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    //               .collect();

    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    //let mut direction = String::new();
    //let mut amount = 0;

    for line in course {
        let unwr_line = line.unwrap();
        let mut split_line = unwr_line.split(' ');

        let direction = split_line.nth(0).unwrap();
        let amount = split_line.nth(0).unwrap().parse::<i64>().unwrap();

        if direction == "forward" {
            forward += amount;
            depth += amount*aim;
        }
        else if direction == "up" {
            aim -= amount;
        }
        else if direction == "down" {
            aim += amount;
        }
    }

    return Ok(forward*depth);
}