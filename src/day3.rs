use std::{collections::HashMap, iter::Peekable, str::Chars};

const METHODS: [&str; 3] = ["mul(", "do(", "don't("];

pub fn solve() {
    let text = std::fs::read_to_string("inputs/day3.txt").unwrap();
    println!("Part 1: {}", multiply(&text));
    println!("Part 2: {}", do_or_dont_multiply(&text));
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
        ans += consume_multiply(&mut it);
    }
    ans
}

fn do_or_dont_multiply(text: &str) -> u32 {
    let mut it = text.chars().peekable();

    let trie = Trie::build(&METHODS);
    let mut enabled = true;
    let mut ans = 0;
    while it.peek().is_some() {
        match trie.find(&mut it) {
            Some("do(") => enabled = true,
            Some("don't(") => enabled = false,
            Some("mul(") if enabled => ans += consume_multiply(&mut it),
            _ => {}
        };
    }
    ans
}

struct Trie<'a> {
    values: HashMap<char, Trie<'a>>,
    matches: Option<&'a str>,
}

impl Trie<'_> {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            matches: None,
        }
    }

    fn build<'a>(words: &[&'a str]) -> Trie<'a> {
        let mut root = Trie::new();
        for word in words {
            let mut node = &mut root;
            for c in word.chars() {
                node = node.values.entry(c).or_insert_with(Trie::new);
            }
            node.matches = Some(word);
        }
        root
    }

    fn find(&self, it: &mut Peekable<Chars>) -> Option<&str> {
        let mut node = match it.next() {
            Some(ch) => match self.values.get(&ch) {
                Some(node) => node,
                None => return None,
            },
            None => return None,
        };
        let mut candidate = None;
        if node.matches.is_some() {
            candidate = node.matches;
        }
        while let Some(ch) = it.peek() {
            node = match node.values.get(ch) {
                Some(node) => {
                    it.next();
                    node
                }
                None => return candidate,
            };
            if node.matches.is_some() {
                candidate = node.matches;
            }
        }
        candidate
    }
}

fn consume_multiply(it: &mut Peekable<Chars>) -> u32 {
    let num1 = match parse_num(it) {
        Some(num) => num,
        None => return 0,
    };
    if !consume_if(it, ',') {
        return 0;
    }
    let num2 = match parse_num(it) {
        Some(num) => num,
        None => return 0,
    };
    if !consume_if(it, ')') {
        return 0;
    }
    num1 * num2
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

    #[test]
    fn test_do_or_dont_multiply() {
        let text = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(do_or_dont_multiply(text), 48);
    }

    #[test]
    fn test_trie_match() {
        let trie = Trie::build(&["a", "abc", "def"]);

        let cases = [
            ("x", None),
            ("a", Some("a")),
            ("ax", Some("a")),
            ("abc", Some("abc")),
            ("abcd", Some("abc")),
            ("de", None),
            ("def", Some("def")),
        ];
        for (haystack, expected) in cases {
            let mut it = haystack.chars().peekable();
            assert_eq!(trie.find(&mut it), expected, "Failed for \"{}\"", haystack);
        }
    }

    #[test]
    fn test_trie_consume() {
        let trie = Trie::build(&["a", "abc", "def"]);

        let haystack = "dexyz";
        let mut it = haystack.chars().peekable();
        assert_eq!(trie.find(&mut it), None);
        assert_eq!(it.next(), Some('x'));

        let haystack = "defgh";
        let mut it = haystack.chars().peekable();
        assert_eq!(trie.find(&mut it), Some("def"));
        assert_eq!(it.next(), Some('g'));

        let haystack = "xyz";
        let mut it = haystack.chars().peekable();
        assert_eq!(trie.find(&mut it), None);
        assert_eq!(it.peek(), Some(&'y'));
        assert_eq!(trie.find(&mut it), None);
        assert_eq!(it.peek(), Some(&'z'));
        assert_eq!(trie.find(&mut it), None);
        assert_eq!(it.peek(), None);
    }
}
