mod submarine;

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_pings() {
        use crate::submarine::sonar;

        let pings = vec![199i64, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let read_pings = sonar::get_pings("test_data.txt").unwrap();

        assert_eq!(pings, read_pings);
    }

    #[test]
    fn test_count_increasing() {
        use crate::submarine::sonar;

        let counts = sonar::count_increasing("test_data.txt").unwrap();

        assert_eq!(counts, 7);
    }

    #[test]
    fn count_increasing_windowsum() {
        use crate::submarine::sonar;

        let counts = sonar::count_increasing_windowsum("test_data.txt").unwrap();

        assert_eq!(counts, 5);
    }
}
