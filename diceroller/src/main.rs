use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    println!("How many dice do you want to roll?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_dice = input.trim().parse::<usize>().unwrap();

    println!("How many sides does each die have?");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let num_sides = input.trim().parse::<usize>().unwrap();

    let mut total = 0;
    for _ in 0..num_dice {
        let roll = rng.gen_range(1..=num_sides);
        total += roll;
        println!("Roll: {}", roll);
    }

    println!("Total: {}", total);
}
