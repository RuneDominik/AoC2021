mod submarine;

use crate::submarine::sonar;

fn main() {
    let count_simple = sonar::count_increasing("../data.txt").unwrap();
    println!("The count is increating {} times", count_simple);

    let count_window = sonar::count_increasing_windowsum("../data.txt").unwrap();
    println!("The count is increating {} times", count_window);
}