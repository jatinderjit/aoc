use std::{collections::HashMap, error::Error, fs};

pub fn solve() {
    let (mut left, mut right) = read_input("inputs/day1.txt").unwrap();
    println!("Part 1: {}", distance(&mut left, &mut right));
    println!("Part 2: {}", similarity(&mut left, &mut right));
}

fn distance(left: &mut [i32], right: &mut [i32]) -> i32 {
    left.sort();
    right.sort();
    left.iter().zip(right).map(|(l, r)| (*l - *r).abs()).sum()
}

fn similarity(left: &mut [i32], right: &mut [i32]) -> i32 {
    let mut counter = HashMap::new();
    for num in right {
        let entry = counter.entry(*num).or_insert(0);
        *entry += 1;
    }
    left.iter()
        .map(|v| v * counter.get(v).copied().unwrap_or(0))
        .sum()
}

fn read_input(input_path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let buf = fs::read_to_string(input_path)?;
    buf.lines()
        .map(|line| {
            line.split("   ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|values| {
            assert!(values.len() == 2);
            left.push(values[0]);
            right.push(values[1]);
        });

    Ok((left, right))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(distance(&mut left, &mut right), 11);
    }

    #[test]
    fn test_similarity() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(similarity(&mut left, &mut right), 31);
    }
}
