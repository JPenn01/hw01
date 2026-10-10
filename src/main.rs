use std::io::{self, Write};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn print_menu() {
    println!("1. Buy food");
    println!("2. Hunt");
    println!("3. Trade with caravan");
    println!("4. View status");
    println!("5. Quit");
}

fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}

fn hunt_food(roll: u64) -> u64 {
    if roll == 1 {
        0
    } else if roll <= 3 {
        10
    } else if roll <= 5 {
        20
    } else {
        40
    }
}

fn read_number(min: u64, max: u64) -> u64 {
    loop {
        print!("Enter a number from {} to {}: ", min, max);
        io::stdout().flush().ok();

        let mut line = String::new();
        io::stdin().read_line(&mut line).ok();

        match line.trim().parse::<u64>() {
            Ok(n) if (min..=max).contains(&n) => return n,
            _ => println!("Please enter a whole number from {} to {}.", min, max),
        }
    }
}

fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=6)
}

fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;
    for i in 0..count {
        total += roll_die(seed + i);
    }
    total
}

fn print_status(gold: u64, food: u64, day: u64) {
    println!("Day {} | Gold: {} | Food: {}", day, gold, food);
}

fn main() {
    print_status(100, 0, 1);
}
