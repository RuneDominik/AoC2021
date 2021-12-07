mod fauna;
mod submarine;

use crate::fauna::crab;
use crate::fauna::lanternfish;
use crate::submarine::diagnostic;
use crate::submarine::piloting;
use crate::submarine::sonar;

fn main() {
    println!("=================== Day 1 ===================");
    let count_simple = sonar::count_increasing("data/day1_data/data.txt").unwrap();
    println!("The simple count is increasing {} times.", count_simple);

    let count_window = sonar::count_increasing_windowsum("data/day1_data/data.txt").unwrap();
    println!("The window count is increasing {} times.", count_window);

    println!("=================== Day 2 ===================");
    let course_auto = piloting::get_course_auto("data/day2_data/data.txt").unwrap();
    println!("The auto course is {}.", course_auto);

    let course_maual = piloting::get_course_manual("data/day2_data/data.txt").unwrap();
    println!("The manual course is {}.", course_maual);

    println!("=================== Day 3 ===================");
    let power_consumption = diagnostic::get_power_consumtion("data/day3_data/data.txt").unwrap();
    println!("The power consumption is {}.", power_consumption);

    let life_support = diagnostic::get_life_support("data/day3_data/data.txt").unwrap();
    println!("The life support counting is {}.", life_support);

    println!("=================== Day 4 ===================");

    println!("=================== Day 5 ===================");

    println!("=================== Day 6 ===================");
    let days80: u16 = 80;
    let fish_80days = //Fish is the actual plural of fish. Fishes would be multiple species of fish. 
        lanternfish::get_lanternfish_population("data/day6_data/data.txt", days80).unwrap();
    println!("There are {} lanternfish after 80 days.", fish_80days);

    let days256: u16 = 256;
    let fish_256days = //Fish is the actual plural of fish. Fishes would be multiple species of fish. 
        lanternfish::get_lanternfish_population("data/day6_data/data.txt", days256).unwrap();
    println!("There are {} lanternfish after 256 days.", fish_256days);

    println!("=================== Day 7 ===================");
    let lin_cons = crab::get_lin_consumption("data/day7_data/data.txt").unwrap();
    println!("There linear fuel consumption is {}.", lin_cons);

    let quad_cons = crab::get_quad_consumption("data/day7_data/data.txt").unwrap();
    println!("There quadratic fuel consumption is {}.", quad_cons);
}
