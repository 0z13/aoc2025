
const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut count = 0;

    for (dx, dy) in OFFSETS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            if grid[ny as usize][nx as usize] {
                count += 1;
            }
        }
    }

    count
}

fn count_grid(grid: &Vec<Vec<bool>>) -> (usize, Vec<Vec<bool>>) {
    let mut next_grid = grid.clone();
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !grid[y][x] {
                continue;
            }
            if count_neighbors(grid, x, y) < 4 {
                count += 1;
                next_grid[y][x] = false;
            }

        }
    }

    (count, next_grid)
}

pub fn part_1(grid: &Vec<Vec<bool>>) -> i32 {
    let mut result = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !grid[y][x] {
                continue;
            }

            if count_neighbors(grid, x, y) < 4 {
                result += 1;
            }
        }
    }
    result
}

pub fn part_2(grid: &Vec<Vec<bool>>) -> usize {
    let mut result: usize = 0;

    let (mut removals, mut next_state) = count_grid(grid);
    result += removals;
    while removals > 0 {
        (removals, next_state) = count_grid(&next_state);
        result += removals;
    }

    result
}
pub fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c == '@')
                .collect()
        })
        .collect()
}
