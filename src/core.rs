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
    let lower = pw.chars().find(|&x| {
        let ascii = x as u8;
        if ascii >= 97 && ascii <= 122 {
            return true;
        }

        false
    });

    if let Some(_) = lower {
        return true;
    }

    false
}

pub fn has_uppercase(pw: &str) -> bool {
    let lower = pw.chars().find(|&x| {
        let ascii = x as u8;
        if ascii >= 65 && ascii <= 90 {
            return true;
        }

        false
    });

    if let Some(_) = lower {
        return true;
    }

    false
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
    fn test_calculate_pool() {
        let mut pool = 0;
        calculate_pool(&mut pool, ")H&7aXvaRt8TE@rj");
        assert_eq!(pool, 94);
    }

    #[test]
    fn test_calculate_entropy_digits_upper_lower_symbols() {
        let mut pool = 0;
        let mut entropy = 0f64;
        let password = "60kRCnKisFY_xua8";
        calculate_pool(&mut pool, password);
        calculate_entropy(&mut entropy, &pool, password);
        assert_eq!(entropy.round(), 105f64);
    }

    #[test]
    fn test_calculate_entropy_digits_upper_lower() {
        let mut pool = 0;
        let mut entropy = 0f64;
        let password = "vootoo0ua1Heikee";
        calculate_pool(&mut pool, password);
        calculate_entropy(&mut entropy, &pool, password);
        assert_eq!(entropy.round(), 95f64);
    }
}
