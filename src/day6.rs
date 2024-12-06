use std::{collections::HashSet, fs::File, io::Read};

pub fn solve() {
    let mut f = File::open("inputs/day6.txt").unwrap();
    let (map, guard) = read_input(&mut f);
    println!("Part 1: {}", cells_visited(&map, guard));
}

fn cells_visited(map: &[Vec<bool>], guard: Guard) -> usize {
    let mut guard = guard;
    let (m, n) = (map.len(), map[0].len());
    let mut visited = HashSet::new();

    loop {
        let (i, j) = match guard.next_cell() {
            Some((i, j)) if i < m && j < n => (i, j),
            _ => break,
        };
        if map[i][j] {
            guard.move_to((i, j));
            visited.insert((i, j));
        } else {
            guard.turn_right();
        }
    }
    visited.len()
}

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Clone, Copy)]
struct Guard {
    pos: Point,
    dir: Dir,
}

impl Guard {
    fn next_cell(&self) -> Option<Point> {
        let (i, j) = self.pos;
        Some(match self.dir {
            Dir::Up if i == 0 => return None,
            Dir::Up => (i - 1, j),
            Dir::Right => (i, j + 1),
            Dir::Down => (i + 1, j),
            Dir::Left if j == 0 => return None,
            Dir::Left => (i, j - 1),
        })
    }

    fn move_to(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn turn_right(&mut self) {
        self.dir = self.dir.turn_right();
    }
}

fn read_input<R: Read>(reader: &mut R) -> (Vec<Vec<bool>>, Guard) {
    let mut bytes = String::new();
    reader.read_to_string(&mut bytes).unwrap();
    let mut guard = None;
    let map = bytes
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| {
                    let (is_walkable, dir) = parse_cell(ch);
                    if let Some(dir) = dir {
                        guard = Some(Guard { pos: (i, j), dir });
                    }
                    is_walkable
                })
                .collect()
        })
        .collect();
    (map, guard.unwrap())
}

fn parse_cell(ch: char) -> (bool, Option<Dir>) {
    match ch {
        '#' => (false, None),
        '.' => (true, None),
        '^' => (true, Some(Dir::Up)),
        '>' => (true, Some(Dir::Right)),
        'v' => (true, Some(Dir::Down)),
        '<' => (true, Some(Dir::Left)),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_input() {
        let inp = "..#.>\n##..#";
        let (map, guard) = read_input(&mut Cursor::new(inp));
        assert_eq!(guard.pos, (0, 4));
        assert_eq!(guard.dir, Dir::Right);
        assert_eq!(
            map,
            vec![
                vec![true, true, false, true, true],
                vec![false, false, true, true, false],
            ]
        );
    }

    #[test]
    fn test_cells_visited() {
        let map = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let (map, guard) = read_input(&mut Cursor::new(map));
        let ans = cells_visited(&map, guard);
        assert_eq!(ans, 41);
    }
}
