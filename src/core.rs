use regex::Regex;

pub fn has_digits(pw: &str) -> bool {
    if pw.chars().any(|x| x.is_digit(10)) {
        return true;
    }
    {
        false
    }
}

pub fn has_lowercase(pw: &str) -> bool {
    if pw.chars().any(|x| x.is_lowercase()) {
        return true;
    }
    {
        false
    }
}

pub fn has_uppercase(pw: &str) -> bool {
    if pw.chars().any(|x| x.is_uppercase()) {
        return true;
    }
    {
        false
    }
}

pub fn has_symbols(pw: &str) -> bool {
    let re = Regex::new(r#"["'`~!@#$%^&*()-_+=[{]}\|;:,.<>/?]"#).unwrap();
    re.is_match(pw)
}

pub fn calculate_pool(pool: &mut usize, password: &str) {
    if has_digits(password) {
        *pool += 10;
    }

    if has_lowercase(password) {
        *pool += 26;
    }

    if has_uppercase(password) {
        *pool += 26;
    }

    if has_symbols(password) {
        *pool += 32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_symbols() {
        assert_eq!(has_symbols("4HIW5e!+MIzVpa5G"), true);
    }

    #[test]
    fn test_has_uppercase() {
        assert_eq!(has_uppercase("uYqqtZiEsifi2Mvu"), true);
    }

    #[test]
    fn test_has_lowercase() {
        assert_eq!(has_lowercase( "VhHw654jxZXJ8Xtj"), true);
    }

    #[test]
    fn test_has_digits() {
        assert_eq!(has_digits( "tqvQLJZ7ESc6o10h"), true);
    }

    #[test]
    fn test_calculate_pool() {
        let mut pool = 0;
        calculate_pool(&mut pool, ")H&7aXvaRt8TE@rj");
        assert_eq!(pool, 94);
    }
}
