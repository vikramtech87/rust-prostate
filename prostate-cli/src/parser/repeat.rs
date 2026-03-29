const MAX_REPEAT: u8 = 6;

pub fn parse_repeat(s: &str) -> Option<u8> {
    let num = s.strip_prefix('x')?
        .parse::<u8>()
        .ok()?;
    
    if (1..=MAX_REPEAT).contains(&num) {
        Some(num)
    } else {
        None    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_invalid_multiplier_format() {
        assert_eq!(parse_repeat("x"), None);
        assert_eq!(parse_repeat("xabc"), None);
        assert_eq!(parse_repeat("2"), None);
    }

    #[test]
    fn reject_out_of_range_multiplier() {
        assert_eq!(parse_repeat("x0"), None);
        assert_eq!(parse_repeat("x7"), None);
    }

    #[test]
    fn parse_valid_repeat() {
        assert_eq!(parse_repeat("x1"), Some(1));
        assert_eq!(parse_repeat("x6"), Some(6));
        assert_eq!(parse_repeat("x4"), Some(4));
    }
}