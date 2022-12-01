use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut largest_calories = 0;
    let mut current_elf_calorie_sum = 0;

    let file = File::open("./input.txt").expect("Unable to open file");
    for raw_line in io::BufReader::new(file).lines() {
        if let Ok(line) = raw_line {
            if line.trim().is_empty() {
                largest_calories = cmp::max(largest_calories, current_elf_calorie_sum);
                current_elf_calorie_sum = 0;
                continue;
            }
            let calorie_item: i32 = line
                .parse()
                .expect(&format!("Could not parse line into int: {}", line));
            current_elf_calorie_sum += calorie_item;
        }
    }
    println!("Most calories: {}", largest_calories)
}
