mod submarine;

use crate::submarine::sonar;
use crate::submarine::piloting;

fn main() {
    println!("=================== Day 1 ===================");
    let count_simple = sonar::count_increasing("/home/runedominik/Desktop/Sonst/AoC2021/aoc2021/day1_data/data.txt").unwrap();
    println!("The simple count is increating {} times", count_simple);

    let count_window = sonar::count_increasing_windowsum("/home/runedominik/Desktop/Sonst/AoC2021/aoc2021/day1_data/data.txt").unwrap();
    println!("The window count is increating {} times", count_window);

    println!("=================== Day 2 ===================");
    let course_auto = piloting::get_course_auto("/home/runedominik/Desktop/Sonst/AoC2021/aoc2021/day2_data/data.txt").unwrap();
    println!("The auto course is {}", course_auto);

    let course_maual = piloting::get_course_manual("/home/runedominik/Desktop/Sonst/AoC2021/aoc2021/day2_data/data.txt").unwrap();
    println!("The manual course is {}", course_maual);

    println!("=================== Day 3 ===================");
}