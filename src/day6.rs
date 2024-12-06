use std::{fs::File, io::Read};

pub fn solve() {
    let mut f = File::open("inputs/day6.txt").unwrap();
    let (map, guard) = read_input(&mut f);
    println!("Part 1: {}", cells_visited(&map, guard));
}

fn cells_visited(map: &[Vec<bool>], guard: Guard) -> usize {
    let mut guard = guard;
    let (m, n) = (map.len(), map[0].len());
    let mut visited = vec![vec![false; n]; m];
    loop {
        let (i, j) = guard.next_cell();
        if i < 0 || i >= m as i32 || j < 0 || j >= n as i32 {
            break;
        }
        if map[i as usize][j as usize] {
            guard.move_to((i, j));
            visited[i as usize][j as usize] = true;
        } else {
            guard.turn_right();
        }
    }
    visited
        .into_iter()
        .map(|row| row.into_iter().filter(|v| *v).count())
        .sum()
}

type Point = (i32, i32);

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
    fn next_cell(&self) -> Point {
        let (i, j) = self.pos;
        match self.dir {
            Dir::Up => (i - 1, j),
            Dir::Right => (i, j + 1),
            Dir::Down => (i + 1, j),
            Dir::Left => (i, j - 1),
        }
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
                        let pos = (i as i32, j as i32);
                        guard = Some(Guard { pos, dir });
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
