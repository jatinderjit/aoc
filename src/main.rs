use std::env::args;

use aoc24::*;

fn main() {
    let days = vec![day1::solve, day2::solve];
    let mut args = args();
    args.next();
    let day = args.next().unwrap().parse::<usize>().unwrap();
    days[day - 1]();
}
