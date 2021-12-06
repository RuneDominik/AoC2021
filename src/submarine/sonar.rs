#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
    path::Path,
};
pub fn get_pings(path: impl AsRef<Path>) -> Result<Vec<i64>, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    return br
        .lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect();
}
pub fn count_increasing(path: impl AsRef<Path>) -> Result<i64, Error> {
    let pings = get_pings(path).unwrap();

    let mut counts = 0;

    for i in 0..pings.len() - 1 {
        if pings[i] < pings[i + 1] {
            counts += 1;
        }
    }

    return Ok(counts);
}
pub fn count_increasing_windowsum(path: impl AsRef<Path>) -> Result<i64, Error> {
    let pings = get_pings(path).unwrap();

    let mut counts = 0;

    for i in 0..pings.len() - 3 {
        if (pings[i] + pings[i + 1] + pings[i + 2]) < (pings[i + 1] + pings[i + 2] + pings[i + 3]) {
            counts += 1;
        }
    }

    return Ok(counts);
}
