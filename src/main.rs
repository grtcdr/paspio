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

        core::calculate_pool(&mut pool, password);

        let entropy = password.len() as f64 * (pool as f64).log2();

        println!("Entropy: {:.2} bits", entropy);
        return;
    }
    {
        println!("No password provided.");
    }
}

