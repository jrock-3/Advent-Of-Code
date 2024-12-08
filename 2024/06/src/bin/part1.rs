use std::collections::BTreeSet;

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
    // dbg!(&map);

    let obstructions = (0..map.len())
        .flat_map(|i| {
            (0..map[i].len())
                .map(move |j| (i, j))
                .filter(|(i, j)| map[*i][*j] == '#')
        })
        .collect::<Vec<_>>();
    // dbg!(&obstructions);

    let mut pos = (0..map.len())
        .flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .filter(|(i, j)| map[*i][*j] == '^')
        .collect::<Vec<_>>()[0];
    // dbg!(&pos);

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
            dir = match dir {
                Dirs::Up => Dirs::Right,
                Dirs::Down => Dirs::Left,
                Dirs::Left => Dirs::Up,
                Dirs::Right => Dirs::Down,
            }
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
