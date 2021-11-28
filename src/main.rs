use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut pool = 0;
    if args.len() > 1 {
        let password = &args[1];
        if password.is_empty() {
            println!("No password provided.");
            return;
        }

        if has_digits(&password) {
            pool += 10;
        }

        if has_lowercase(&password) {
            pool += 26;
        }

        if has_uppercase(&password) {
            pool += 26;
        }

        if has_symbols(&password) {
            pool += 32;
        }

        let entropy = password.len() as f64 * (pool as f64).log2();

        println!("Entropy: {:.2} bits", entropy);
        return;
    }
    {
        println!("No password provided.");
    }
}

fn has_digits(pw: &String) -> bool {
    if pw.chars().any(|x| x.is_digit(10)) {
        return true;
    }
    {
        false
    }
}

fn has_lowercase(pw: &String) -> bool {
    if pw.chars().any(|x| x.is_lowercase()) {
        return true;
    }
    {
        false
    }
}

fn has_uppercase(pw: &String) -> bool {
    if pw.chars().any(|x| x.is_uppercase()) {
        return true;
    }
    {
        false
    }
}

fn has_symbols(pw: &String) -> bool {
    let re = Regex::new(r#"["'`~!@#$%^&*()-_+=[{]}\|;:,.<>/?]"#).unwrap();
    re.is_match(pw)
}
