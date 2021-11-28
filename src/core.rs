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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_symbols() {
        let pw = String::from("4HIW5e!+MIzVpa5G");
        assert_eq!(has_symbols(&pw), true);
    }

    #[test]
    fn test_has_uppercase() {
        let pw = String::from("uYqqtZiEsifi2Mvu");
        assert_eq!(has_uppercase(&pw), true);
    }

    #[test]
    fn test_has_lowercase() {
        let pw = String::from("VhHw654jxZXJ8Xtj");
        assert_eq!(has_lowercase(&pw), true);
    }

    #[test]
    fn test_has_digits() {
        let pw = String::from("tqvQLJZ7ESc6o10h");
        assert_eq!(has_digits(&pw), true);
    }
}
