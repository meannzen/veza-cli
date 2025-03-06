pub fn generate_stop_id(pattern: &str, index: usize) -> String {
    let prefix_end = pattern.chars().take_while(|c| !c.is_ascii_digit()).count();
    let (prefix, num_part) = pattern.split_at(prefix_end);

    match num_part.parse::<usize>() {
        Ok(num) => {
            let new_num = num + index + 1;
            format!("{}{:0width$}", prefix, new_num, width = num_part.len())
        }
        Err(_) => {
            // If num_part isn't a valid number, use prefix and append index + 1 without padding
            format!("{}{}", prefix, index + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_increment() {
        assert_eq!(generate_stop_id("ST000000", 0), "ST000001");
        assert_eq!(generate_stop_id("ST000000", 1), "ST000002");
        assert_eq!(generate_stop_id("ST000000", 9), "ST000010");
    }

    #[test]
    fn test_different_prefix() {
        assert_eq!(generate_stop_id("ABC123", 0), "ABC124");
        assert_eq!(generate_stop_id("XYZ000", 1), "XYZ002");
        assert_eq!(generate_stop_id("T-", 2), "T-3"); // No digits in pattern
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(generate_stop_id("ST000000", 99), "ST000100");
        assert_eq!(generate_stop_id("ST000", 0), "ST001");
        assert_eq!(generate_stop_id("ST00", 9), "ST10");
    }

    #[test]
    fn test_no_digits_in_pattern() {
        assert_eq!(generate_stop_id("STOP", 0), "STOP1");
        assert_eq!(generate_stop_id("ID-", 5), "ID-6");
    }

    #[test]
    fn test_large_index() {
        assert_eq!(generate_stop_id("ST000000", 999), "ST001000");
        assert_eq!(generate_stop_id("ST000000", 10000), "ST010001");
    }

    #[test]
    fn test_empty_pattern() {
        assert_eq!(generate_stop_id("", 0), "1");
        assert_eq!(generate_stop_id("", 5), "6");
    }
}
