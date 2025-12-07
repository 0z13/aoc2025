use std::ops::{ RangeInclusive};


#[derive(Default)]
pub struct Inventory {
    ranges: Vec<RangeInclusive<usize>>,
    food_items: Vec<usize>,
}

pub fn part_2(inventory: &Inventory) -> usize {
    use std::cmp::max;
    let mut ranges = inventory.ranges.clone();
    ranges.sort_by_key(|x| *x.start());
    let mut v = vec![];
    
    let mut current = ranges[0].clone();

    for range in ranges.into_iter().skip(1) {
        if range.start() <= current.end() {
            let new_end = *max(current.end(), range.end());
            current = *current.start()..=new_end;
        } else {
            v.push(current);
            current = range;
        }
    }
    
    v.push(current);

    let mut result = 0;
    // all disjoint
    for i in v {
        // [a,b] -> (b-a)+1
        result += i.end() - i.start() + 1;
    }
    
    result
}


pub fn part_1(inventory: &Inventory) -> usize {
    let mut counter = 0;
    for i in &inventory.food_items {
        for r in &inventory.ranges {
            if r.contains(&i) {
                counter += 1;
                break;
            }
        }
        
    }

    counter
}


pub fn parse(_input: &str) -> Inventory {
    let mut inventory = Inventory::default();
    let mut iter = _input.trim_end().split('\n');

    for r in &mut iter {
        if r == "" {
            break;
        }
        let (start, end) = r.split_once('-').expect("-");
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();
        let range = RangeInclusive::new(start,end);
        inventory.ranges.push(range);
        
    }

    for num in iter {
        let num_parsed = num.parse::<usize>().unwrap();
        inventory.food_items.push(num_parsed);
    }
    
    inventory
}
