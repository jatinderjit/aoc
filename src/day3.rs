use std::{iter::Peekable, str::Chars};

pub fn solve() {
    let text = std::fs::read_to_string("inputs/day3.txt").unwrap();
    println!("Part 1: {}", multiply(&text));
}

fn multiply(text: &str) -> u32 {
    let mut it = text.chars().peekable();
    let mut ans = 0;
    while let Some(ch) = it.next() {
        if ch != 'm' {
            continue;
        }
        if !consume_if(&mut it, 'u') || !consume_if(&mut it, 'l') || !consume_if(&mut it, '(') {
            continue;
        }
        let num1 = match parse_num(&mut it) {
            Some(num) => num,
            None => continue,
        };
        if !consume_if(&mut it, ',') {
            continue;
        }
        let num2 = match parse_num(&mut it) {
            Some(num) => num,
            None => continue,
        };
        if !consume_if(&mut it, ')') {
            continue;
        }
        ans += num1 * num2;
    }
    ans
}

fn parse_num(it: &mut Peekable<Chars>) -> Option<u32> {
    let mut num = 0;
    let mut is_valid = false;
    while let Some(Some(d)) = it.peek().map(|c| c.to_digit(10)) {
        num = num * 10 + d;
        it.next();
        is_valid = true;
    }
    if is_valid {
        Some(num)
    } else {
        None
    }
}

fn consume_if(it: &mut Peekable<Chars>, val: char) -> bool {
    match it.peek() {
        Some(ch) if *ch == val => {
            it.next();
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply() {
        let text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(multiply(text), 161);
    }
}
