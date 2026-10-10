use std::io::{self, Write};

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

fn main() {
    let n = read_number(1, 10);
    println!("You entered {}", n);
}
