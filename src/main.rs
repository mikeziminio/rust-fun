extern crate rand;

use std::{io, cmp};
use rand::Rng;

fn main() {

    let secret: u32 = rand::thread_rng().gen_range(0..100);

    loop {
        println!("\nВведите число: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = match input.trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => continue
        };

        match secret.cmp(&input) {
            cmp::Ordering::Greater => println!("Больше"),
            cmp::Ordering::Less => println!("Меньше"),
            cmp::Ordering::Equal => {
                println!("Выиграл!");
                break;
            }
        }
    }
}
