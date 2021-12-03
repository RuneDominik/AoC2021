mod submarine;

#[cfg(test)]
mod sonar_tests {
    #[test]
    fn test_get_pings() {
        use crate::submarine::sonar;

        let pings = vec![199i64, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let read_pings = sonar::get_pings("day1_data/test_data.txt").unwrap();

        assert_eq!(pings, read_pings);
    }

    #[test]
    fn test_count_increasing() {
        use crate::submarine::sonar;

        let counts = sonar::count_increasing("day1_data/test_data.txt").unwrap();

        assert_eq!(counts, 7);
    }

    #[test]
    fn count_increasing_windowsum() {
        use crate::submarine::sonar;

        let counts = sonar::count_increasing_windowsum("day1_data/test_data.txt").unwrap();

        assert_eq!(counts, 5);
    }
}

#[cfg(test)]
mod piloting_tests {
    #[test]
    fn test_get_course_auto() {
        use crate::submarine::piloting;

        let course = piloting::get_course_auto("day2_data/test_data.txt").unwrap();

        assert_eq!(course, 150);
    }
    #[test]
    fn test_get_course_manual() {
        use crate::submarine::piloting;

        let course = piloting::get_course_manual("day2_data/test_data.txt").unwrap();

        assert_eq!(course, 900);
    }
}

#[cfg(test)]
mod diagnostic_tests {
    #[test]
    fn test_get_power_consumtion() {
        use crate::submarine::diagnostic;

        let power_consumption = diagnostic::get_power_consumtion("day3_data/test_data.txt").unwrap();

        assert_eq!(power_consumption, 198);
    }
}
