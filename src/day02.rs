use std::{ops::RangeInclusive};
// I'll do it in the dumbest possible way first,
// and revisit later. No good ideas right now.
pub fn part_1(ranges: &Vec<RangeInclusive<usize>>) -> usize {
    let mut counter = 0;
    for range in ranges {
        for i in range.clone() {
            let s = i.to_string();
            let (a,b) = s.split_at(s.len() / 2);
            if a == b {
                counter += i;
            }

        }
    }
    counter
}

pub fn part_2(ranges: &Vec<RangeInclusive<usize>>) -> usize {
    1337
}

pub fn parse(input: &'static str) -> Vec<RangeInclusive<usize>> {
    let mut ranges = vec![];

    for substr in input.trim_end().split(',') {
        let idx = substr.find('-').expect("Expecting: -");
        let (start, end) = substr.split_at(idx);
        let r = start.parse().unwrap()..=end[1..].parse().unwrap();
        ranges.push(r);
    }

    ranges
}
