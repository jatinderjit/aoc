use std::{collections::HashMap, fs::File, io::Read};

pub fn solve() {
    let mut f = File::open("inputs/day6.txt").unwrap();
    let (map, guard) = read_input(&mut f);

    match cells_visited(&map, guard) {
        GuardPath::Visited(visited) => println!("Part 1: {visited}"),
        GuardPath::InLoop => unreachable!(),
    };
    println!("Part 2: {}", num_pos_for_loop(&map, guard));
}

#[derive(Debug, PartialEq)]
enum GuardPath {
    Visited(usize),
    InLoop,
}

fn cells_visited(map: &[Vec<bool>], guard: Guard) -> GuardPath {
    let mut guard = guard;
    let (m, n) = (map.len(), map[0].len());
    let mut visited = HashMap::new();
    visited.insert(guard.pos, vec![guard.dir]);
    loop {
        let (i, j) = match guard.next_cell() {
            Some((i, j)) if i < m && j < n => (i, j),
            _ => return GuardPath::Visited(visited.len()),
        };
        if map[i][j] {
            let entry = visited.entry((i, j)).or_insert_with(Vec::new);
            if entry.contains(&guard.dir) {
                return GuardPath::InLoop;
            }
            entry.push(guard.dir);
            guard.move_to((i, j));
        } else {
            guard.turn_right();
        }
    }
}

// Uses brute force
fn num_pos_for_loop(map: &[Vec<bool>], guard: Guard) -> usize {
    let mut count = 0;
    let mut map = map.to_vec();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] && guard.pos != (i, j) {
                println!("Testing: ({i}, {j})");
                map[i][j] = false;
                count += match cells_visited(&map, guard) {
                    GuardPath::Visited(_) => 0,
                    GuardPath::InLoop => 1,
                };
                map[i][j] = true;
            }
        }
    }
    count
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
        assert_eq!(ans, GuardPath::Visited(41));
    }

    #[test]
    fn test_num_pos_for_loop() {
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
        let ans = num_pos_for_loop(&map, guard);
        assert_eq!(ans, 6);
    }
}
