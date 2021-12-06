mod submarine;
mod fauna;

#[cfg(test)]
mod sonar_tests {
    #[test]
    fn test_get_pings() {
        use crate::submarine::sonar;

        let pings = vec![199i64, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let read_pings = sonar::get_pings("data/day1_data/test_data.txt").unwrap();

        assert_eq!(pings, read_pings);
    }

    #[test]
    fn test_count_increasing() {
        use crate::submarine::sonar;

        let counts = sonar::count_increasing("data/day1_data/test_data.txt").unwrap();

        assert_eq!(counts, 7);
    }

    #[test]
    fn count_increasing_windowsum() {
        use crate::submarine::sonar;

        let counts = sonar::count_increasing_windowsum("data/day1_data/test_data.txt").unwrap();

        assert_eq!(counts, 5);
    }
}

#[cfg(test)]
mod piloting_tests {
    #[test]
    fn test_get_course_auto() {
        use crate::submarine::piloting;

        let course = piloting::get_course_auto("data/day2_data/test_data.txt").unwrap();

        assert_eq!(course, 150);
    }
    #[test]
    fn test_get_course_manual() {
        use crate::submarine::piloting;

        let course = piloting::get_course_manual("data/day2_data/test_data.txt").unwrap();

        assert_eq!(course, 900);
    }
}

#[cfg(test)]
mod diagnostic_tests {
    #[test]
    fn test_get_power_consumtion() {
        use crate::submarine::diagnostic;

        let power_consumption =
            diagnostic::get_power_consumtion("data/day3_data/test_data.txt").unwrap();

        assert_eq!(power_consumption, 198);
    }
    #[test]
    fn test_get_life_support() {
        use crate::submarine::diagnostic;

        let life_support = diagnostic::get_life_support("data/day3_data/test_data.txt").unwrap();

        assert_eq!(life_support, 230);
    }
}

#[cfg(test)]
mod lanternfish_tests {
    #[test]
    fn test_get_lanternfish_population() {
        use crate::fauna::lanternfish2;

        let days18: u16 = 18;
        let days80: u16 = 80;

        let fish_18days = //Fish is the actual plural of fish. Fishes would be multiple species of fish. 
            lanternfish2::get_lanternfish_population("data/day6_data/test_data.txt", days18).unwrap();

        let fish_80days = 
            lanternfish2::get_lanternfish_population("data/day6_data/test_data.txt", days80).unwrap();

        assert_eq!(fish_18days, 26);
        assert_eq!(fish_80days, 5934);
    }
}
