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
        if (32..=47).contains(&ascii) {
            true
        } else if (58..=64).contains(&ascii) {
            true
        } else if (91..=96).contains(&ascii) {
            true
        } else if (123..=126).contains(&ascii) {
            true
        } else {
            false
        }
    });

    if lower.is_some() {
        return true;
    }

    false
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
        *pool += 33;
    }
}

pub fn calculate_entropy(entropy: &mut f64, pool: &usize, password: &str) {
    *entropy = password.len() as f64 * (*pool as f64).log2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_symbols() {
        assert_eq!(has_symbols("JGA7Y#ELWX,8$QB3"), true);
        assert_eq!(has_symbols("uVR,8EX2^_F95j?M"), true);
        assert_eq!(has_symbols("GODzi[wCnw4{of]\\"), true);
        assert_eq!(has_symbols("yj<qNo9hu=\"<<c3`"), true);
        assert_eq!(has_symbols("u=x@;qLZ\\m)RY-7d"), true);
    }

    #[test]
    fn test_has_uppercase() {
        assert_eq!(has_uppercase(".xNDoUKZM2X\"M.tJ"), true);
        assert_eq!(has_uppercase("{w:cN8t79lSLkB|O"), true);
        assert_eq!(has_uppercase("_N{L*%u_r%<4'+fb"), true);
        assert_eq!(has_uppercase("XVf}mos,tt03|!n="), true);
        assert_eq!(has_uppercase("ba8@j#!h|C)Ccr}4"), true);
    }

    #[test]
    fn test_has_lowercase() {
        assert_eq!(has_lowercase("baeAKaaWeshAjoje"), true);
        assert_eq!(has_lowercase("aechoWmeeyuXeeHb"), true);
        assert_eq!(has_lowercase("ohhBSuiphievohsh"), true);
        assert_eq!(has_lowercase("ahNefeesoofuNeih"), true);
        assert_eq!(has_lowercase("eifeiNUuzSucheez"), true);
    }

    #[test]
    fn test_has_digits() {
        assert_eq!(has_digits("_rGp5W||XJ!N5z\"6"), true);
        assert_eq!(has_digits("W?k*U0\"'z^43WMFi"), true);
        assert_eq!(has_digits("+Te;S0fXS0;t4yU1"), true);
        assert_eq!(has_digits("uHDN1zX$g[J>WH\\V"), true);
        assert_eq!(has_digits("zMaZXu8>d}k8tlu3"), true);
    }

    #[test]
    fn test_calculate_pool_with_digits() {
        let mut pool = 0;
        calculate_pool(&mut pool, "12983791");
        assert_eq!(pool, 10);
    }

    #[test]
    fn test_calculate_pool_with_upper_lower() {
        let mut pool = 0;
        calculate_pool(&mut pool, "Aokeethi");
        assert_eq!(pool, 52);
    }

    #[test]
    fn test_calculate_pool_with_symbols() {
        let mut pool = 0;
        calculate_pool(&mut pool, "-_+\\`%{]");
        assert_eq!(pool, 33);
    }

    #[test]
    fn test_calculate_entropy_with_upper_lower() {
        let mut pool = 0;
        let mut entropy = 0f64;
        let password = "Cievabad";
        calculate_pool(&mut pool, password);
        calculate_entropy(&mut entropy, &pool, password);
        assert_eq!(entropy.round(), 46f64);
    }

    #[test]
    fn test_calculate_entropy_with_symbols() {
        let mut pool = 0;
        let mut entropy = 0f64;
        let password = "&;`_/`\\@";
        calculate_pool(&mut pool, password);
        calculate_entropy(&mut entropy, &pool, password);
        assert_eq!(entropy.round(), 40f64);
    }

    #[test]
    fn test_calculate_entropy_with_digits() {
        let mut pool = 0;
        let mut entropy = 0f64;
        let password = "02172341";
        calculate_pool(&mut pool, password);
        calculate_entropy(&mut entropy, &pool, password);
        assert_eq!(entropy.round(), 27f64);
    }
}
