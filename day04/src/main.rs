use std::fs::File;
use std::io::{self, BufRead};

struct Range {
    lower: u32,
    upper: u32,
}

impl Range {
    fn new(raw_range: String) -> Range {
        let mut split = raw_range.split('-');
        Range {
            lower: split
                .next()
                .expect("Could not parse lower bound")
                .parse::<u32>()
                .unwrap(),
            upper: split
                .next()
                .expect("Could not parse upper bound")
                .parse::<u32>()
                .unwrap(),
        }
    }

    fn overlaps(&self, other: Range) -> bool {
        other.lower <= self.lower && other.upper >= self.upper
            || self.lower <= other.lower && self.upper >= other.upper
    }
}

fn main() {
    let mut full_overlaps = 0;
    let file = File::open("./input.txt").expect("Could not open the input file");
    for raw_line in io::BufReader::new(file).lines() {
        let line = raw_line.unwrap();
        let mut assignments = line.split(',');

        let first_elf_work = assignments
            .next()
            .expect("Could not get first elf's assignments");
        let second_elf_work = assignments
            .next()
            .expect("Could not get second elf's assignments");

        let range1 = Range::new(first_elf_work.to_string());
        let range2 = Range::new(second_elf_work.to_string());

        if range1.overlaps(range2) {
            full_overlaps += 1;
        }
    }
    println!("{}", full_overlaps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_range() {
        let range = Range::new("2-5".to_string());
        assert_eq!(range.lower, 2);
        assert_eq!(range.upper, 5);
    }

    #[test]
    fn test_range_overlap() {
        let range1 = Range::new("2-5".to_string());
        let range2 = Range::new("3-4".to_string());
        assert_eq!(range1.overlaps(range2), true);

        let range3 = Range::new("2-5".to_string());
        let range4 = Range::new("6-6".to_string());
        assert_eq!(range3.overlaps(range4), false);
    }
}
