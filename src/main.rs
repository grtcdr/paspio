fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let password = &args[1];
        if password.is_empty() {
            print!("No password provided.");
            return;
        }

        let pool = paspio::get_pool_size(password);
        let entropy = paspio::get_entropy(pool, password);

        print!("Entropy: {:.2} bits", entropy);
    } else {
        print!("No password provided.");
    }
}
