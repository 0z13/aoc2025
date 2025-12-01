
#[derive(Debug)]
pub enum Direction {
    Left,
    Right
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    value: i32,
}

pub fn parse(input: &str ) -> Vec<Instruction> {
    let mut values = vec![];
    for instr in input.trim_end().split('\n') {
        let (d,v) = instr.split_at(1);
        let direction = match d {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("at the disco"),
        };

        let value: i32 = v.parse().unwrap();
        values.push(Instruction {
            direction,
            value,
        })
    }
    values
}

pub fn modulo(x: i32, y: i32) -> i32 {
    let remainder = x % y;
    if remainder < 0 {
        y + remainder
    } else {
        remainder
    }
}

pub fn part_1(instructions: &Vec<Instruction>) -> usize {
    use Direction::*;
    let mut dial = 50;
    let mut count = 0;
    for instr in instructions {
        match instr.direction {
            Left => {
                dial = modulo(dial - instr.value, 100);
            },
            Right => {
                dial = modulo(dial + instr.value, 100);
            }
        }
        if dial == 0 {
            count += 1;
        }
    }
    count
}

pub fn part_2(instructions: &Vec<Instruction>) -> usize {
    use Direction::*;
    let mut dial: i32 = 50;
    let mut count = 0;

    for instr in instructions {
        // count any full rotations first
        count += (instr.value / 100) as usize;

        let remainder = instr.value % 100;

        // count remaining
        if remainder > 0 {
            match instr.direction {
                Right => {
                    if dial + remainder >= 100 {
                        count += 1;
                    }
                dial = modulo(dial - instr.value, 100);
                },
                Left => {
                    if dial > 0 && (dial - remainder <= 0) {
                        count += 1;
                    }
                    dial -= remainder;
                    if dial < 0 {
                        dial += 100;
                    }
                }
            }
        }
    }
    count
}

