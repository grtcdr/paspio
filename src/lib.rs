/// Returns the pool size of a given password, e.g.:
///
/// **Pool** of `Pepperoni` is:
/// - [a-z]
/// - [A-Z]
///
/// **Pool size** of `Pepperoni` is 26 + 26 = 52
fn get_pool_size(password: &str) -> usize {
    let mut pool = 0;

    if password.chars().any(|c| c.is_ascii_digit()) {
        pool += 10;
    }

    if password.chars().any(|c| c.is_ascii_lowercase()) {
        pool += 26;
    }

    if password.chars().any(|c| c.is_ascii_uppercase()) {
        pool += 26;
    }

    if password
        .chars()
        .any(|c| c.is_ascii_punctuation() || c.is_ascii_whitespace())
    {
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
/// assert_eq!(entropy.round(), 60f64); // in bits
/// ```
pub fn get_entropy(password: &str) -> f64 {
    password.len() as f64 * (get_pool_size(password) as f64).log2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_pool_with_upper() {
        let password = "AOKEETHI";
        let pool = get_pool_size(password);
        assert_eq!(pool, 26);
    }

    #[test]
    fn test_calculate_pool_with_lower() {
        let password = "aokeethi";
        let pool = get_pool_size(password);
        assert_eq!(pool, 26);
    }

    #[test]
    fn test_calculate_pool_with_digits() {
        let password = "12983791";
        let pool = get_pool_size(password);
        assert_eq!(pool, 10);
    }

    #[test]
    fn test_calculate_pool_with_symbols() {
        let password = "-_+\\`%{]";
        let pool = get_pool_size(password);
        assert_eq!(pool, 33);
    }

    #[test]
    fn test_calculate_pool_with_upper_lower_digits_symbols() {
        let password = "bO9I9=./<a:K4}~N";
        let pool = get_pool_size(password);
        assert_eq!(pool, 95);
    }

    #[test]
    fn test_calculate_entropy_with_upper() {
        let password = "TAGXEEDAIE";
        let entropy = get_entropy(password);
        assert_eq!(entropy.round(), 47f64);
    }

    #[test]
    fn test_calculate_entropy_with_lower() {
        let password = "cievabadop";
        let entropy = get_entropy(password);
        assert_eq!(entropy.round(), 47f64);
    }

    #[test]
    fn test_calculate_entropy_with_digits() {
        let password = "0217234181";
        let entropy = get_entropy(password);
        assert_eq!(entropy.round(), 33f64);
    }

    #[test]
    fn test_calculate_entropy_with_symbols() {
        let password = "&;`_/`\\@$#";
        let entropy = get_entropy(password);
        assert_eq!(entropy.round(), 50f64);
    }

    #[test]
    fn test_calculate_entropy_with_upper_lower_digits_symbols() {
        let password = "zFOuR@/nb9";
        let entropy = get_entropy(password);
        assert_eq!(entropy.round(), 66f64);
    }
}
