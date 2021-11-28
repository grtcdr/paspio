fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.is_empty() {
        print!("No password provided.");
        return;
    }

    for password in &args {
        let entropy = paspio::get_entropy(password);
        println!("{:.2} bits", entropy);
    }
}
