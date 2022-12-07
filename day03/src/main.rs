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
    for raw_line in io::BufReader::new(file).lines() {
        let mut first_half = raw_line.unwrap();
        let second_half = first_half.split_off(first_half.len() / 2);

        let first_set: HashSet<char> = HashSet::from_iter(first_half.chars());
        let second_set: HashSet<char> = HashSet::from_iter(second_half.chars());

        let mut intersect = first_set.intersection(&second_set);
        let item = intersect.next().unwrap();
        summed_priorities += item_priority(*item);
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
