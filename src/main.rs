mod core;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let password = &args[1];
        if password.is_empty() {
            println!("No password provided.");
            return;
        }

        let mut pool = 0;
        let mut entropy = 0f64;

        core::calculate_pool(&mut pool, password);
        core::calculate_entropy(&mut entropy, &pool, password);

        println!("Entropy: {:.2} bits", entropy);
    } else {
        println!("No password provided.");
    }
}
