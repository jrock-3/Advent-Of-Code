use std::collections::BTreeSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

#[derive(Debug, Clone, Copy)]
enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

impl Dirs {
    fn rotate(self) -> Self {
        match self {
            Dirs::Up => Dirs::Right,
            Dirs::Down => Dirs::Left,
            Dirs::Left => Dirs::Up,
            Dirs::Right => Dirs::Down,
        }
    }
    fn get_pos(self, pos: (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Dirs::Up => (pos.0.checked_sub(1)?, pos.1),
            Dirs::Down => (pos.0.checked_add(1)?, pos.1),
            Dirs::Left => (pos.0, pos.1.checked_sub(1)?),
            Dirs::Right => (pos.0, pos.1.checked_add(1)?),
        })
    }
}

fn pos_in_bounds(map: &Vec<Vec<char>>, pos: &(usize, usize)) -> bool {
    (0..map.len()).contains(&pos.0) && (0..map[0].len()).contains(&pos.1)
}

fn process(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let obstructions = (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|&(i, j)| map[i][j] == '#')
        .collect::<Vec<_>>();

    let mut pos = (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|&(i, j)| map[i][j] == '^')
        .collect::<Vec<_>>()[0];

    let mut dir = Dirs::Up;
    let mut visited = BTreeSet::new();
    while pos_in_bounds(&map, &pos) {
        let next = dir.get_pos(pos);
        if next.is_none() {
            break;
        }

        let next = next.unwrap();
        if !pos_in_bounds(&map, &next) {
            break;
        }

        if !obstructions.contains(&next) {
            visited.insert(next);
            pos = next;
        } else {
            dir = dir.rotate();
        }
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!("41", process(input));
    }
}
