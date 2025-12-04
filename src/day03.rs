

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
pub fn part_2(data: &Vec<Vec<u32>>) -> u64 {
    let mut v = vec![];

    for bat in data {
        let mut right_start = 0;
        let mut search = 12;
        let mut result: u64 = 0;

        while search > 0 {
            let end_bound = bat.len() - search + 1;
            let slice = &bat[right_start..end_bound];

            let (new_index, value) = slice.iter()
                .enumerate()
                .rev() // max_by_key returns the _last_ maximum number on ties,
                       // we want the first to increase window
                .max_by_key(|(_idx, val)| *val)
                .unwrap();

            result = result * 10 + (*value as u64);

            right_start += new_index + 1;
            search -= 1;
        }
        v.push(result);
    }
    v.iter().sum()
}

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
      .trim_end()
      .split_terminator('\n')
      .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
      .collect()
}
