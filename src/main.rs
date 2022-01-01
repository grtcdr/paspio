use std::io;

fn main() {
    let stdin = io::stdin();
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if !args.is_empty() {
        args.iter().for_each(|pass| {
            print_entropy(pass);
        });
    } else {
        let mut passwords: Vec<String> = vec![];
        loop {
            let mut pass = String::new();
            if let Ok(len) = stdin.read_line(&mut pass) {
                // Ctrl+D breaks out of loop.
                if len == 0 {
                    break;
                }

                // Skip over blank lines.
                if pass.as_bytes().iter().all(u8::is_ascii_whitespace) {
                    continue;
                }

                passwords.push(pass);
            }
        }

        if passwords.is_empty() {
            println!("No passwords provided.");
            return;
        }

        passwords.iter().for_each(|pass| {
            print_entropy(pass);
        })
    }
}

fn print_entropy(password: &str) {
    let entropy = paspio::get_entropy(password);
    println!("{:.2} bits", entropy);
}
