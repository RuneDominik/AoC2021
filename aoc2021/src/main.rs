mod submarine;

use crate::submarine::diagnostic;
use crate::submarine::piloting;
use crate::submarine::sonar;
use crate::fauna::lanternfish;

fn main() {
    println!("=================== Day 1 ===================");
    let count_simple = sonar::count_increasing("aoc2021/day1_data/data.txt").unwrap();
    println!("The simple count is increating {} times", count_simple);

    let count_window = sonar::count_increasing_windowsum("aoc2021/day1_data/data.txt").unwrap();
    println!("The window count is increating {} times", count_window);

    println!("=================== Day 2 ===================");
    let course_auto = piloting::get_course_auto("aoc2021/day2_data/data.txt").unwrap();
    println!("The auto course is {}", course_auto);

    let course_maual = piloting::get_course_manual("aoc2021/day2_data/data.txt").unwrap();
    println!("The manual course is {}", course_maual);

    println!("=================== Day 3 ===================");
    let power_consumption = diagnostic::get_power_consumtion("aoc2021/day3_data/data.txt").unwrap();
    println!("The auto course is {}", power_consumption);

    let life_support = diagnostic::get_life_support("aoc2021/day3_data/data.txt").unwrap();
    println!("The auto course is {}", life_support);

    println!("=================== Day 4 ===================");

    println!("=================== Day 5 ===================");

    println!("=================== Day 6 ===================");
    let days80: u8 = 80;
    let fish_80days = //Fish is the actual plural of fish. Fishes would be multiple species of fish. 
        lanternfish::get_lanternfish_population("day6_data/data.txt", days80).unwrap();
    println!("There are {} lanternfish after 80 days.", fish_80days)
}
