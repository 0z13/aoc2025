mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::time::Instant;

macro_rules! display_day {
    ($day_module:ident, $input_file:expr) => {
        {
            println!("\n============== {} ==============", stringify!($day_module));

            let input = include_str!(concat!("../data/", $input_file));

            let t0 = Instant::now();
            let parsed_input = $day_module::parse(input);
            let t_parse = t0.elapsed();
            println!("Parsing: {:.2?}", t_parse);

            let t1 = Instant::now();
            let result_1 = $day_module::part_1(&parsed_input);
            let t_p1 = t1.elapsed();
            println!("Part 1:  {} ({:.2?})", result_1, t_p1);

            let t2 = Instant::now();
            let result_2 = $day_module::part_2(&parsed_input);
            let t_p2 = t2.elapsed();
            println!("Part 2:  {} ({:.2?})", result_2, t_p2);
            println!("Total:   {:.2?}", t_parse + t_p1 + t_p2);
            println!("===================================");
        }
    };
}

fn main() {
    // display_day!(day01, "1.in");
    // display_day!(day02, "2.in");
    // display_day!(day03, "3.in");
    // display_day!(day04, "4.in");
    // display_day!(day05, "5.in");
    display_day!(day06, "6.in");
}
