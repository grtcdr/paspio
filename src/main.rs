fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.is_empty() {
        print!("No password provided.");
        return;
    }

    for password in &args {
        let pool = paspio::get_pool_size(password);
        let entropy = paspio::get_entropy(pool, password);
        println!("{:.2} bits", entropy);
    }
}
