
pub fn part_1(_s: &str) -> usize {
    3
}


pub fn part_2(_s: &str) -> usize {
    3
}

enum Op {
    Mult,
    Plus,    
}

struct OctupusMath {
    nums: Vec<i32>,
    Op: Op,
}

pub fn parse(s: &str) -> &str {
    // parse stuff

    for line in s.lines() {
        println!("{}", line);
    }
    
    "ehllo"
} 
