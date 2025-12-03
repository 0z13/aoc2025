use std::{ops::RangeInclusive};
use std::collections::HashSet;
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

fn invalid_ids(seed_limit: usize, max: usize) -> HashSet<usize> {
    let mut all_invalid_ids = HashSet::new();

    for n in 1..seed_limit {
        let id = n.to_string();

        let mut current = id.clone();

        loop {
            current.push_str(&id);

            if current.len() > max.to_string().len() {
                break;
            }

            let id_val = current.parse::<usize>().expect("heyho");

            if id_val > max {
                break;
            }

            all_invalid_ids.insert(id_val);
        }
    }

    all_invalid_ids
}

pub fn part_2(ranges: &Vec<RangeInclusive<usize>>) -> usize {
    let max = ranges.iter()
        .map(|range| range.end())
        .max()
        .unwrap();

    let strlen = max.to_string().len();

    let seed_limit = 10_usize.pow((strlen as u32 / 2) + 2);

    let h = invalid_ids(seed_limit, *max);

    let mut result = 0;
    for invalid_id in h {
        for range in ranges {
            if range.contains(&invalid_id) {
                result += invalid_id;
                break;
            }
        }
    }

    result
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
