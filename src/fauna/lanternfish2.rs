#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

struct School {
    school: Vec<u64>,
}

impl School {
    pub fn new(ages: Vec<i8>) -> School {
        let mut school = vec![0; 9];
        School::init_school(&mut school, ages);
        School { school: school}
    }

    fn init_school(school: &mut Vec<u64>, ages: Vec<i8>) {
        for age in ages {
            school[age as usize] += 1;
        }
    }

    fn advance_one_day(school: &mut Vec<u64>) {
       let have_spawned: u64 = school[0];
       school.rotate_left(1);
       school[6] += have_spawned;
    }

    pub fn advance(&mut self, days: u16){
        for day in 0..days {
            if day % 10 == 0 {
                println!("The day is {}", day);
            }
            School::advance_one_day(&mut self.school);
        }
    }

    pub fn count(&self) -> u64 {
        let mut i: u64 = 0;
        for val in &self.school {
            i += val;
        }
        return i;

    }
}
pub fn get_lanternfish_population(path: impl AsRef<Path>, days: u16) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let ages = br.lines().next().unwrap().unwrap().split(",").map(|x| x.parse::<i8>().unwrap()).collect();

    let mut population = School::new(ages);
    population.advance(days);

    return Ok(population.count());
}
