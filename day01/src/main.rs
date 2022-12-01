use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Default)]
struct TopThree(i32, i32, i32);

impl TopThree {
    fn update(&mut self, new_value: i32) {
        self.2 = cmp::min(self.1, cmp::max(self.2, new_value));
        self.1 = cmp::min(self.0, cmp::max(self.1, new_value));
        self.0 = cmp::max(self.0, new_value);
    }
    fn total(&self) -> i32 {
        self.0 + self.1 + self.2
    }
}

impl fmt::Display for TopThree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "1: {}\n 2: {}\n 3: {}\n Total: {}",
            self.0,
            self.1,
            self.2,
            self.total()
        )
    }
}

fn main() {
    let mut top_three: TopThree = Default::default();
    let mut current_elf_calorie_sum = 0;

    let file = File::open("./input.txt").expect("Unable to open file");
    for raw_line in io::BufReader::new(file).lines() {
        if let Ok(line) = raw_line {
            if line.trim().is_empty() {
                top_three.update(current_elf_calorie_sum);
                current_elf_calorie_sum = 0;
                continue;
            }
            let calorie_item: i32 = line
                .parse()
                .expect(&format!("Could not parse line into int: {}", line));
            current_elf_calorie_sum += calorie_item;
        }
    }
    println!("Top calories:\n{}", top_three)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut top_three: TopThree = Default::default();
        top_three.update(1);
        top_three.update(0);
        top_three.update(2);
        top_three.update(3);
        top_three.update(2);
        top_three.update(4);
        assert_eq!(top_three.total(), 4 + 3 + 2);
    }
}
