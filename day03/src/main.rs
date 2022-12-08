#![feature(iter_next_chunk)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn item_priority(item: char) -> u32 {
    // A: 65, Z: 90 ( sub 64 )=> A: 1, Z: 26 ( add 26 )=> 27 to 52
    // a: 97, a: 122 ( sub (97 - 27) )=> a: 27, z: 52 ( sub 26 )=> 1 to 26
    let ascii_val = item as u32;
    if ascii_val <= 90 {
        return ascii_val - 38;
    }
    return ascii_val - 96;
}

fn main() {
    let mut summed_priorities = 0;
    let file = File::open("./input.txt").expect("Could not open input file");
    let mut lines = io::BufReader::new(file).lines().peekable();

    while Option::is_some(&lines.peek()) {
        let [ruck1, ruck2, ruck3] = lines.next_chunk().unwrap();
        let ruck_set_1: HashSet<char> = HashSet::from_iter(ruck1.unwrap().chars());
        let ruck_set_2: HashSet<char> = HashSet::from_iter(ruck2.unwrap().chars());
        let ruck_set_3: HashSet<char> = HashSet::from_iter(ruck3.unwrap().chars());

        let intersection = ruck_set_1
            .iter()
            .filter(|e| ruck_set_2.contains(e) && ruck_set_3.contains(e))
            .next()
            .expect("Could not find common element in group");

        summed_priorities += item_priority(*intersection);
    }
    println!("{}", summed_priorities);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
    }
}
