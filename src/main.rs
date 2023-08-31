extern crate rand;
use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.contains(&"--help".to_string()) {
        help()
    }
    
    let size: i32 = match args[1].parse() {
        Ok(parsed_size) => parsed_size,
        Err(_) => {
            println!("Invalid integer argument provided.");
            return;
        }
    };
    let symbols: &String = &args[2];
    
    match symbols.as_str() {
        "-y" => {
            let char_range = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%ˆ&*()_+=˜;:<>?[]|";
            let mut rng = rand::thread_rng();
            
            let pass: String = (0..size)
                .map(|_| {
                let randindex = rng.gen_range(0..char_range.len());
                char_range.chars().nth(randindex).unwrap()
                })
                .collect();
            println!("{}", pass)
        }
        "-n" => {
            let char_range = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            let mut rng = rand::thread_rng();
            
            let pass: String = (0..size)
                .map(|_| {
                let randindex = rng.gen_range(0..char_range.len());
                char_range.chars().nth(randindex).unwrap()
                })
                .collect();
                println!("{}", pass)
        }
        _ => {
            help()
        }
    };
}

fn help() {
    println!("Usage: password-generator 10 -y");
}
