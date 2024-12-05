use std::{cmp::Ordering, fs::File, io::Read};

pub fn solve() {
    let mut f = File::open("inputs/day5.txt").unwrap();
    let (deps, updates) = read_input(&mut f);
    println!("Part 1: {}", correctly_ordered(&deps, &updates));
    println!("Part 2: {}", incorrectly_ordered(&deps, &updates));
}

fn correctly_ordered(deps: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| is_update_correctly_ordered(deps, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn incorrectly_ordered(deps: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| !is_update_correctly_ordered(deps, update))
        .map(|update| reorder(deps, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn reorder(deps: &[(u32, u32)], update: &[u32]) -> Vec<u32> {
    let mut update = update.to_vec();
    update.sort_by(|num1, num2| {
        if deps.contains(&(*num1, *num2)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    update
}

fn is_update_correctly_ordered(deps: &[(u32, u32)], update: &[u32]) -> bool {
    for i in 1..update.len() {
        if !is_pair_correctly_ordered(deps, update[i - 1], update[i]) {
            return false;
        }
    }
    true
}

fn is_pair_correctly_ordered(deps: &[(u32, u32)], num1: u32, num2: u32) -> bool {
    assert!(deps.contains(&(num1, num2)) || deps.contains(&(num2, num1)));
    deps.contains(&(num1, num2))
}

fn read_input<R: Read>(reader: &mut R) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut text = String::new();
    reader.read_to_string(&mut text).unwrap();

    let mut lines = text.lines();

    let mut deps = vec![];
    let mut updates = vec![];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut pair = line.split('|').map(|s| s.parse().unwrap());
        let num1 = pair.next().unwrap();
        let num2 = pair.next().unwrap();
        deps.push((num1, num2));
    }

    for line in lines {
        let nums = line.split(',').map(|s| s.parse().unwrap()).collect();
        updates.push(nums);
    }
    (deps, updates)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    const TEST_TEXT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_read_input() {
        let text = "1|2\n3|1\n3|2\n\n1,2,3\n2,1,3\n3,1,2";
        let (deps, updates) = read_input(&mut Cursor::new(text));
        assert_eq!(deps, vec![(1, 2), (3, 1), (3, 2)]);
        assert_eq!(updates, vec![vec![1, 2, 3], vec![2, 1, 3], vec![3, 1, 2]]);
    }

    #[test]
    fn test_correctly_ordered() {
        let (deps, updates) = read_input(&mut Cursor::new(TEST_TEXT));
        assert_eq!(correctly_ordered(&deps, &updates), 143);
    }

    #[test]
    fn test_incorrectly_ordered() {
        let (deps, updates) = read_input(&mut Cursor::new(TEST_TEXT));
        assert_eq!(incorrectly_ordered(&deps, &updates), 123);
    }
}
