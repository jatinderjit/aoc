use std::env::args;

use aoc24::*;

fn main() {
    let days = [
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
    ];
    let mut args = args();
    args.next();
    let day = args.next().unwrap().parse::<usize>().unwrap();
    days[day - 1]();
}
