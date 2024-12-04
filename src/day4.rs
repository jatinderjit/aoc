pub fn solve() {
    let text = std::fs::read_to_string("inputs/day4.txt").unwrap();
    let grid = to_grid(text.lines());
    println!("Part 1: {}", count_xmas(&grid));
    println!("Part 2: {}", count_cross_mas(&grid));
}

type Grid = Vec<Vec<char>>;
type Word = Vec<char>;
type Point = (i32, i32);

fn to_grid<'a, I>(lines: I) -> Grid
where
    I: IntoIterator<Item = &'a str>,
{
    lines.into_iter().map(to_word).collect()
}

fn to_word(text: &str) -> Word {
    text.chars().collect()
}

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_xmas(grid: &Grid) -> usize {
    let mut ans = 0;
    let xmas = to_word("XMAS");
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            ans += count_words_at(grid, &xmas, (x as i32, y as i32));
        }
    }
    ans
}

fn count_cross_mas(grid: &Grid) -> usize {
    let mut count = 0;
    let pairs = [('M', 'S'), ('S', 'M')];
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            if grid[i][j] == 'A' {
                let p1 = (grid[i - 1][j - 1], grid[i + 1][j + 1]);
                let p2 = (grid[i + 1][j - 1], grid[i - 1][j + 1]);
                if pairs.contains(&p1) && pairs.contains(&p2) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn exists_in_dir(grid: &Grid, word: &Word, start: Point, dir: Point) -> bool {
    let (mut x, mut y) = start;
    for ch in word {
        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
            return false;
        }

        if *ch != grid[x as usize][y as usize] {
            return false;
        }

        x += dir.0;
        y += dir.1;
    }
    true
}

fn count_words_at(grid: &Grid, word: &Word, start: Point) -> usize {
    if grid[start.0 as usize][start.1 as usize] != word[0] {
        return 0;
    }
    if word.len() == 1 {
        return 1;
    }
    let mut count = 0;
    for dir in DIRS {
        if exists_in_dir(grid, word, start, dir) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exists_in_dir() {
        let grid = to_grid(["ABC", "ABC", "ABC"]);

        assert!(exists_in_dir(&grid, &to_word("A"), (0, 0), (1, 1)));
        assert!(exists_in_dir(&grid, &to_word("AB"), (0, 0), (0, 1)));
        assert!(exists_in_dir(&grid, &to_word("AB"), (0, 0), (1, 1)));
        assert!(!exists_in_dir(&grid, &to_word("AB"), (0, 0), (-1, 1)));
    }

    #[test]
    fn test_count_words_at() {
        let grid = to_grid(["ABCDE", "ABCDE"]);
        assert_eq!(count_words_at(&grid, &to_word("X"), (0, 0)), 0);
        assert_eq!(count_words_at(&grid, &to_word("A"), (0, 0)), 1);
        assert_eq!(count_words_at(&grid, &to_word("AB"), (0, 0)), 2);
    }

    #[test]
    fn test_count_xmas() {
        let rows = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        assert_eq!(count_xmas(&to_grid(rows)), 18);
    }

    #[test]
    fn test_count_cross_mas() {
        let rows = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        assert_eq!(count_cross_mas(&to_grid(rows)), 9);
    }
}
