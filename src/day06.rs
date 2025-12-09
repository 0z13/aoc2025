
pub fn part_1(m: &Math) -> usize {
    let mut result = 0;
    let mut tmp = 0;
    for i in 0..m.nums[0].len() {
        if m.ops[i] == Op::Mult {
            tmp = 1;
        } else {
            tmp = 0;
        }
        for j in 0..m.nums.len() {
            match m.ops[i] {
                Op::Mult => {
                   tmp *= m.nums[j][i];
                },
                Op::Plus => {
                    tmp += m.nums[j][i];
                },
            }
        }
        result += tmp;
        tmp = 0;
    }
    result
}


pub fn part_2(_s: &Math) -> usize {
    3
}

#[derive(Default, PartialEq)]
pub enum Op {
    #[default]
    Mult,
    Plus,    
}

#[derive(Default)]
pub struct Math {
    nums: Vec<Vec<usize>>,
    ops: Vec<Op>,
}

pub fn parse(s: &str) -> Math {
    let mut row = vec![];
    let mut ops: Vec<Op> = vec![];
    let mut accum: Option<usize> = None;
    let mut nums = vec![vec![]];

    // could just split_whitespace(), but yolo
    for ch in s.chars() {
        if let Some(digit) = ch.to_digit(10) {
            let new = accum.unwrap_or(0) * 10 + (digit as usize);
            accum = Some(new);
        } else if ch.is_whitespace() {
            if let Some(num) = accum {
                row.push(num);
                accum = None;
            }
            if ch == '\n' {
                nums.push(row);
                row = vec![];
            }
        } else {
            match ch {
                '*' => {
                    let op = Op::Mult;
                    ops.push(op);
                },
                '+' => {
                    let op = Op::Plus;
                    ops.push(op);
                },
                _ => panic!("Unexpected token"),
            }
        }
    }

    // sigh
    let nums: Vec<Vec<usize>> = nums.into_iter().filter(|x| !x.is_empty()).collect();

    for case in &nums {
        assert!(case.len() == ops.len());
    }

    Math {
        nums,
        ops,
    }
} 
