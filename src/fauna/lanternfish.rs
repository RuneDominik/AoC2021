#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};
use std::fmt;

#[derive(Clone, Copy)]
struct Fish {
    time_till_spawn: i8
}

struct School {
    school: Vec<Fish>,
}

impl<'a> IntoIterator for &'a School {
    type Item = Fish;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Fish>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.school.iter().copied()
    }
}


impl Fish {
    fn age(&mut self) -> i8 {
        let ret:i8 =  self.time_till_spawn;
        if self.time_till_spawn == 0 {
            self.time_till_spawn = 6;
        }
        else {
            self.time_till_spawn -= 1;
        }
        return ret;
        }
    }

impl fmt::Display for Fish {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let tmp = self.time_till_spawn.to_string();

        formatter.pad_integral(self.time_till_spawn >= 0, "Foo ", &tmp)
    }
}

impl School {
    pub fn new(ages: Vec<i8>) -> School {
        let mut school = Vec::new();
        School::init_school(&mut school, ages);
        School { school: school}
    }

    fn init_school(school: &mut Vec<Fish>, ages: Vec<i8>) {
        for age in ages {
            school.push(Fish {time_till_spawn: age})
        }
    }

    fn spawn(school: &mut Vec<Fish>, i:i64) {
        if i > 0 {
            for _j in 0..i {
                school.push(Fish {time_till_spawn: 8})
            }
        }
    }

    fn advance_one_day(school: &mut Vec<Fish>) {
        let mut i: i64 = 0;

        let ages = school.into_iter().map(|x| x.age()).collect::<Vec<_>>();
        
        for a in ages {
            if a == 0 {
                i += 1;
            }
        }

        School::spawn(school, i);
    }

    pub fn advance(&mut self, days: u8){
        for _day in 0..days {
            School::advance_one_day(&mut self.school);
        }
    }

    pub fn count(&self) -> i64 {
        let mut i: i64 = 0;
        for _f in &self.school {
            i += 1;
        }
        return i;
    }

}

pub fn get_lanternfish_population(path: impl AsRef<Path>, days: u8) -> Result<i64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let ages = br.lines().next().unwrap().unwrap().split(",").map(|x| x.parse::<i8>().unwrap()).collect();

    let mut population = School::new(ages);
    population.advance(days);

    return Ok(population.count());
}