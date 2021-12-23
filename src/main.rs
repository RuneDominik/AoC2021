mod fauna;
mod geology;
mod submarine;

use crate::fauna::crab;
use crate::fauna::lanternfish;
use crate::geology::lavatubes;
use crate::submarine::diagnostic;
use crate::submarine::piloting;
use crate::submarine::sonar;
use crate::submarine::syntax;
use crate::submarine::thermal_camera;
use crate::submarine::reactor;

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

    println!("=================== Day 8 ===================");

    println!("=================== Day 9 ===================");
    let risk_level = lavatubes::get_risk_level("data/day9_data/data.txt").unwrap();
    println!("There risk level is {}.", risk_level);

    let basin_size = lavatubes::get_basin_size("data/day9_data/data.txt").unwrap();
    println!("There multiplied basin size is {}.", basin_size);

    println!("=================== Day 10 ===================");
    let error_score = syntax::get_corrupted_lines("data/day10_data/data.txt").unwrap();
    println!("There error score is {}.", error_score);

    let completion_score = syntax::get_completed_lines("data/day10_data/data.txt").unwrap();
    println!("There completion score is {}.", completion_score);

    println!("=================== Day 11 ===================");

    println!("=================== Day 12 ===================");

    println!("=================== Day 13 ===================");
    let n_points_first_fold = thermal_camera::get_first_fold("data/day13_data/data.txt").unwrap();
    println!(
        "There are {} points after the first fold.",
        n_points_first_fold
    );
    let n_points_all_fold = thermal_camera::get_code("data/day13_data/data.txt").unwrap();
    println!("There are {} points after all folds.", n_points_all_fold);

    println!("=================== Day 14 ===================");

    println!("=================== Day 15 ===================");

    println!("=================== Day 16 ===================");

    println!("=================== Day 17 ===================");

    println!("=================== Day 18 ===================");

    println!("=================== Day 19 ===================");

    println!("=================== Day 20 ===================");

    println!("=================== Day 21 ===================");

    println!("=================== Day 22 ===================");
    let active_cubes = reactor::initialize_reactor("data/day22_data/data.txt").unwrap();
    println!(
        "There are {} cubes activated.",
        active_cubes
    );

    //let restarted_cubes = reactor::restart_reactor("data/day22_data/test_data.txt").unwrap();
    //println!(
    //    "There are {} cubes activated.",
    //    restarted_cubes
    //);

    println!("=================== Day 23 ===================");

    println!("=================== Day 24 ===================");
}
