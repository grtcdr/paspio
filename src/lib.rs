/// Returns true if a given password contains at least one digit.
fn has_digits(pw: &str) -> bool {
    pw.chars().any(|x| x.is_digit(10))
}

/// Returns true if a given password contains at least one lowercase letter.
fn has_lowercase(pw: &str) -> bool {
    pw.chars()
        .find(|&x| (97..=122).contains(&(x as u8)))
        .is_some()
}

/// Returns true if a given password contains at least one uppercase letter.
fn has_uppercase(pw: &str) -> bool {
    pw.chars()
        .find(|&x| (65..=90).contains(&(x as u8)))
        .is_some()
}

/// Returns true if a given password contains at least one symbol.
fn has_symbols(pw: &str) -> bool {
    pw.chars()
        .find(|&x| {
            let ascii = x as u8;
            (32..=47).contains(&ascii)
                || (58..=64).contains(&ascii)
                || (91..=96).contains(&ascii)
                || (123..=126).contains(&ascii)
        })
        .is_some()
}

/// Returns the pool size of a given password, e.g.:
///
/// **Pool** of `Pepperoni` is:
/// - [a-z]
/// - [A-Z]
///
/// **Pool size** of `Pepperoni` is 26 + 26 = 52
fn get_pool_size(password: &str) -> usize {
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

/// Returns the entropy of a given password.
///
/// `Entropy = L * log2(R)`
///
/// Where:
/// - `L` is the length of the password.
/// - `R` is the pool size of the password.
/// 
/// # Example
///
/// ```
/// let entropy = paspio::get_entropy("quain1aeLu");
/// assert_eq!(entropy.round(), 33f64); // in bits
/// ```
pub fn get_entropy(password: &str) -> f64 {
    password.len() as f64 * (get_pool_size(password) as f64).log2()
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
        let pool = get_pool_size("12983791");
        assert_eq!(pool, 10);
    }

    #[test]
    fn test_calculate_pool_with_upper_lower() {
        let pool = get_pool_size("Aokeethi");
        assert_eq!(pool, 52);
    }

    #[test]
    fn test_calculate_pool_with_symbols() {
        let pool = get_pool_size("-_+\\`%{]");
        assert_eq!(pool, 33);
    }

    #[test]
    fn test_calculate_entropy_with_upper_lower() {
        let password = "Cievabad";
        let entropy = get_entropy(password);
        assert_eq!(entropy.round(), 46f64);
    }

    #[test]
    fn test_calculate_entropy_with_symbols() {
        let password = "&;`_/`\\@";
        let entropy = get_entropy(password);
        assert_eq!(entropy.round(), 40f64);
    }

    #[test]
    fn test_calculate_entropy_with_digits() {
        let password = "02172341";
        let entropy = get_entropy(password);
        assert_eq!(entropy.round(), 27f64);
    }
}
