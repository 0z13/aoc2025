

pub fn part_1(data: &Vec<Vec<u32>>)  -> usize {
    let mut max_left = 0;
    let mut right_start = 0;
    let mut result = 0;
    for bat in data {
        for (idx, val) in bat[..bat.len()-1].iter().enumerate() {
            if max_left < *val {
                right_start = idx;
                max_left = *val;
            }
        }
        let r = bat[right_start+1..].iter().max().unwrap();
        result += ((max_left*10)+r) as usize;
        right_start = 0;
        max_left = 0;
    }
    result
}

pub fn part_2(data: &Vec<Vec<u32>>) -> usize{

    1337
}

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
      .trim_end()
      .split_terminator('\n')
      .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
      .collect()
}
