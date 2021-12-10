#![allow(dead_code)]

use itertools::{rev, sorted};
use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
    path::Path,
};

fn find_corrupted_lines(line: Vec<&str>) -> Result<i64, Error> {
    let mut expected_chars = Vec::new();

    let opening = vec!["(", "[", "{", "<"];

    for current in line {
        if opening.contains(&current) {
            let expected = match current {
                "(" => ")",
                "[" => "]",
                "{" => "}",
                "<" => ">",
                _ => return Err(Error::new(ErrorKind::Other, "Unknown char.")),
            };

            expected_chars.push(expected);
        } else {
            let expected = expected_chars.pop();

            if expected.is_none() {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Line starts with a closing bracket.",
                ));
            } else if expected != Some(current) {
                let score: i64 = match current {
                    ")" => 3,
                    "]" => 57,
                    "}" => 1197,
                    ">" => 25137,
                    _ => return Err(Error::new(ErrorKind::Other, "Unknown char.")),
                };
                return Ok(score);
            }
        }
    }
    if !expected_chars.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            "There were unclosed brackets.",
        ));
    }
    return Err(Error::new(ErrorKind::Other, "Not Implemented."));
}

fn repair_incomplete_lines(line: Vec<&str>) -> Result<i64, Error> {
    let mut expected_chars = Vec::new();

    let opening = vec!["(", "[", "{", "<"];

    for current in line {
        if opening.contains(&current) {
            let expected = match current {
                "(" => ")",
                "[" => "]",
                "{" => "}",
                "<" => ">",
                _ => return Err(Error::new(ErrorKind::Other, "Unknown char.")),
            };

            expected_chars.push(expected);
        } else {
            let expected = expected_chars.pop();
            if expected.is_none() {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Line starts with a closing bracket.",
                ));
            } else if expected != Some(current) {
                return Err(Error::new(ErrorKind::Other, "Corrupt Line"));
            }
        }
    }

    if !expected_chars.is_empty() {
        let mut score: i64 = 0;
        for bracket in rev(expected_chars.into_iter()) {
            score *= 5;
            score += match bracket {
                ")" => 1,
                "]" => 2,
                "}" => 3,
                ">" => 4,
                _ => 0,
            };
        }
        return Ok(score);
    }

    return Err(Error::new(ErrorKind::Other, "Not Implemented."));
}

pub fn get_corrupted_lines(path: impl AsRef<Path>) -> Result<i64, Error> {
    // read file and get line-length and count
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let lines = br.lines().map(|x| x.unwrap());

    let score = lines
        .map(|x| {
            find_corrupted_lines(x.split("").filter(|x| x != &"").collect::<Vec<_>>()).unwrap_or(0)
        })
        .sum();
    return Ok(score);
}

pub fn get_completed_lines(path: impl AsRef<Path>) -> Result<i64, Error> {
    // read file and get line-length and count
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let lines = br.lines().map(|x| x.unwrap());

    let scores = lines
        .map(|x| {
            repair_incomplete_lines(x.split("").filter(|x| x != &"").collect::<Vec<_>>())
                .unwrap_or(0)
        })
        .filter(|x| *x != 0);

    let sorted_scores: Vec<i64> = rev(sorted(scores)).collect();

    let score: i64 = sorted_scores[(sorted_scores.len() / 2) as usize];

    return Ok(score);
}
