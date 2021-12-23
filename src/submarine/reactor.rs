#![allow(dead_code)]

use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

type Bounds = (isize, isize);

type Cube = (Bounds, Bounds, Bounds);

struct ActionCube {
    activate: bool,
    cube: Cube,
}

pub fn initialize_reactor(path: impl AsRef<Path>) -> Result<usize, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let lines = br.lines().map(|x| x.unwrap());

    let mut active_cubes = HashSet::new();

    for line in lines {
        if line.contains("on") {
            let substring = line
                .replace("on ", "")
                .replace("x=", "")
                .replace("y=", "")
                .replace("z=", "");
            let directions: Vec<&str> = substring.split(",").collect();
            let x_range: Vec<i32> = directions[0]
                .split("..")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let y_range: Vec<i32> = directions[1]
                .split("..")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let z_range: Vec<i32> = directions[2]
                .split("..")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if x_range[0] > 50
                || x_range[1] < -50
                || y_range[0] > 50
                || y_range[1] < -50
                || z_range[0] > 50
                || z_range[1] < -50
            {
                continue;
            }

            for x in x_range[0]..x_range[1] + 1 {
                for y in y_range[0]..y_range[1] + 1 {
                    for z in z_range[0]..z_range[1] + 1 {
                        active_cubes.insert(vec![x, y, z]);
                    }
                }
            }
        } else if line.contains("off") {
            let substring = line
                .replace("off ", "")
                .replace("x=", "")
                .replace("y=", "")
                .replace("z=", "");
            let directions: Vec<&str> = substring.split(",").collect();
            let x_range: Vec<i32> = directions[0]
                .split("..")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let y_range: Vec<i32> = directions[1]
                .split("..")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let z_range: Vec<i32> = directions[2]
                .split("..")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if x_range[0] > 50
                || x_range[1] < -50
                || y_range[0] > 50
                || y_range[1] < -50
                || z_range[0] > 50
                || z_range[1] < -50
            {
                continue;
            }

            for x in x_range[0]..x_range[1] + 1 {
                for y in y_range[0]..y_range[1] + 1 {
                    for z in z_range[0]..z_range[1] + 1 {
                        active_cubes.remove(&vec![x, y, z]);
                    }
                }
            }
        }
    }

    return Ok(active_cubes.len());
}

fn intersection(left: &Cube, right: &Cube) -> Option<Cube> {
    let c = (
        (left.0 .0.max(right.0 .0), left.0 .1.min(right.0 .1)),
        (left.1 .0.max(right.1 .0), left.1 .1.min(right.1 .1)),
        (left.2 .0.max(right.2 .0), left.2 .1.min(right.2 .1)),
    );
    if c.0 .0 <= c.0 .1 && c.1 .0 <= c.1 .1 && c.2 .0 <= c.2 .1 {
        Some(c)
    } else {
        None
    }
}

/*
pub fn restart_reactor(path: impl AsRef<Path>) -> Result<usize, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let mut cubes: Vec<ActionCube> = br.lines().map(|x| x.unwrap()).map(|x| {
        let substring = x.split_once(' ').unwrap();
        let activate: bool = match substring.0 {
            "on" => true,
            "off" => false,
            _ => unreachable!(),
        };
        let mut pos = substring.1.split(',');
        let (x1, x2) = pos.next().unwrap().split_once("..").unwrap().map(|x| x.parse::<i32>().unwrap());
        let (y1, y2) = pos.next().unwrap().split_once("..").unwrap().map(|x| x.parse::<i32>().unwrap());
        let (z1, z2) = pos.next().unwrap().split_once("..").unwrap().map(|x| x.parse::<i32>().unwrap());

        ActionCube(activate: activate, cube: Cube(Bounds(x1, x2), Bounds(y1, y2), Bounds(z1, z2)))
    }).collect();

    let mut result = Vec::<ActionCube>::new();
    for c in cubes {
        let mut added = Vec::new();
        if c.activate {
            added.push(*c)
        }
        for existing_cube in result {
            if let Some(intersected_cube) = intersection(cube, &existing_cube.cube) {
                added.push(ActionCube {
                    activate: !existing_cube.activate,
                    cube: intersected_cube,
                });
            }
        }
        result.extend(added);
    }

    return Ok(0);

}
*/
