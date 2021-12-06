#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Fish {
    time_till_spawn: i8
}

struct School {
    school: Vec<Fish>,
}

// impl<'a> IntoIterator for &'a School {
//     type Item = Fish;
//     type IntoIter = std::iter::Copied<std::slice::Iter<'a, Fish>>;
//     
//     fn into_iter(self) -> Self::IntoIter {
//         self.school.iter().copied()
//     }
// }

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

    fn spawn(school: &mut Vec<Fish>, i:usize) {
        let mut new_fish = vec![Fish {time_till_spawn: 8}; i];
        school.append(&mut new_fish);
    }

    fn advance_one_day(school: &mut Vec<Fish>) {
        let i = school.into_par_iter().map(|x| x.age()).filter(|&n| n == 0).count();
        School::spawn(school, i);
    }

    pub fn advance(&mut self, days: u16){
        for day in 0..days {
            if day % 10 == 0 {
                println!("The day is {}", day);
            }
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

pub fn get_lanternfish_population(path: impl AsRef<Path>, days: u16) -> Result<i64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let ages = br.lines().next().unwrap().unwrap().split(",").map(|x| x.parse::<i8>().unwrap()).collect();

    let mut population = School::new(ages);
    population.advance(days);

    return Ok(population.count());
}
