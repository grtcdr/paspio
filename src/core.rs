pub fn has_digits(pw: &str) -> bool {
    if pw.chars().any(|x| x.is_digit(10)) {
        return true;
    }
    {
        false
    }
}

pub fn has_lowercase(pw: &str) -> bool {
    let lower = pw.chars().find(|&x| {
        let ascii = x as u8;
        if (97..=122).contains(&ascii) {
            return true;
        }

        false
    });

    if lower.is_some() {
        return true;
    }

    false
}

pub fn has_uppercase(pw: &str) -> bool {
    let lower = pw.chars().find(|&x| {
        let ascii = x as u8;
        if (65..=90).contains(&ascii) {
            return true;
        }

        false
    });

    if lower.is_some() {
        return true;
    }

    false
}

pub fn has_symbols(pw: &str) -> bool {
    let lower = pw.chars().find(|&x| {
        let ascii = x as u8;
        (32..=47).contains(&ascii)
        || (58..=64).contains(&ascii)
        || (91..=96).contains(&ascii)
        || (123..=126).contains(&ascii)
    });

    if lower.is_some() {
        return true;
    }

    false
}

pub fn get_pool(password: &str) -> usize {
    let mut pool = 0;

    if has_digits(password) {
        pool += 10;
    }

    if has_lowercase(password) {
        pool += 26;
    }

    if has_uppercase(password) {
        pool += 26;
    }

    if has_symbols(password) {
        pool += 33;
    }

    pool
}

pub fn get_entropy(pool: usize, password: &str) -> f64 {
    password.len() as f64 * (pool as f64).log2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_symbols() {
        assert_eq!(has_symbols("JGA7Y#ELWX,8$QB3"), true);
        assert_eq!(has_symbols("uVR28EX2caF95jjM"), false);
    }

    #[test]
    fn test_has_uppercase() {
        assert_eq!(has_uppercase(".xNDoUKZM2X\"M.tJ"), true);
        assert_eq!(has_uppercase("{w:cn8t79lslkb|0"), false);
    }

    #[test]
    fn test_has_lowercase() {
        assert_eq!(has_lowercase("baeAKaaWeshAjoje"), true);
        assert_eq!(has_lowercase("AECHOWMEEYUXEEHB"), false);
    }

    #[test]
    fn test_has_digits() {
        assert_eq!(has_digits("_rGp5W||XJ!N5z\"6"), true);
        assert_eq!(has_digits("W?k*UO\"'z^AEWMFi"), false);
    }

    #[test]
    fn test_calculate_pool_with_digits() {
        let pool = get_pool("12983791");
        assert_eq!(pool, 10);
    }

    #[test]
    fn test_calculate_pool_with_upper_lower() {
        let pool = get_pool("Aokeethi");
        assert_eq!(pool, 52);
    }

    #[test]
    fn test_calculate_pool_with_symbols() {
        let pool = get_pool("-_+\\`%{]");
        assert_eq!(pool, 33);
    }

    #[test]
    fn test_calculate_entropy_with_upper_lower() {
        let password = "Cievabad";
        let pool = get_pool(password);
        let entropy = get_entropy(pool, password);
        assert_eq!(entropy.round(), 46f64);
    }

    #[test]
    fn test_calculate_entropy_with_symbols() {
        let password = "&;`_/`\\@";
        let pool = get_pool(password);
        let entropy = get_entropy(pool, password);
        assert_eq!(entropy.round(), 40f64);
    }

    #[test]
    fn test_calculate_entropy_with_digits() {
        let password = "02172341";
        let pool = get_pool(password);
        let entropy = get_entropy(pool, password);
        assert_eq!(entropy.round(), 27f64);
    }
}
