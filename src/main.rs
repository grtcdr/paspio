fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(password) = &args.get(1) {
        let pool = paspio::get_pool_size(password);
        let entropy = paspio::get_entropy(pool, password);

        print!("Entropy: {:.2} bits", entropy);
    }

    print!("No password provided.");
}
