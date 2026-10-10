use std::io::{self, Write};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;
const CARAVAN_DICE: u64 = 3;
const TRADE_MULTIPLIER: u64 = 2;


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
    let seed = read_number(1, MAX_SEED);
    let mut gold = START_GOLD;
    let mut food: u64 = 0;
    let mut day: u64 = 1;
    let mut roll_count: u64 = 0;

    loop {
        print_menu();
        let choice = read_number(1, 5);

        match choice {
            1 => {
                if !can_afford(gold, FOOD_PRICE) {
                    println!("You can't afford any food.");
                    continue;
                }
                let max_units = gold / FOOD_PRICE;
                let units = read_number(1, max_units);
                gold -= units * FOOD_PRICE;
                food += units;
                day += 1;
            }
            2 => {
                let roll = roll_die(seed + roll_count);
                roll_count += 1;
                let gained = hunt_food(roll);
                food += gained;
                day += 1;
                println!("You rolled a {} and gained {} food.", roll, gained);
            }
            3 => {
                let total = roll_many(seed + roll_count, CARAVAN_DICE);
                roll_count += CARAVAN_DICE;
                let earned = TRADE_MULTIPLIER * total;
                gold += earned;
                day += 1;
                println!("The caravan trade earned you {} gold.", earned);
            }
            4 => print_status(gold, food, day),
            _ => {
                print_status(gold, food, day);
                break;
            }
        }
    }
}
