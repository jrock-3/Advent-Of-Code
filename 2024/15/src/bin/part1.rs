use std::collections::BTreeSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn print_map(
    xlen: usize,
    ylen: usize,
    position: (i32, i32),
    walls: &BTreeSet<(i32, i32)>,
    boxes: &BTreeSet<(i32, i32)>,
) {
    println!(
        "{}\n",
        (0..xlen as i32)
            .map(|i| {
                let walls = walls.clone();
                let boxes = boxes.clone();
                (0..ylen as i32)
                    .map(move |j| {
                        if walls.contains(&(i, j)) {
                            '#'
                        } else if boxes.contains(&(i, j)) {
                            'O'
                        } else if (i, j) == position {
                            '@'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .join("\n")
    );
}

fn process(input: &str) -> String {
    let xlen = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .collect_vec()
        .len();
    let ylen = input.lines().next().unwrap().len();
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = map.lines().map(|line| line.chars());

    let mut walls = BTreeSet::new();
    let mut boxes = BTreeSet::new();
    let mut position = (-1, -1);

    for (i, row) in map.enumerate() {
        print_map(xlen, ylen, position, &walls, &boxes);
        let i = i as i32;
        for (j, dir) in row.enumerate() {
            let j = j as i32;
            match dir {
                '#' => {
                    walls.insert((i, j));
                }
                'O' => {
                    boxes.insert((i, j));
                }
                '@' => {
                    position = (i, j);
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    // print_map(xlen, ylen, &walls, &boxes);

    for dir in moves.chars().filter(|&c| c != '\n') {
        // print_map(xlen, ylen, position, &walls, &boxes);
        let (dx, dy) = match dir {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => unreachable!(),
        };

        let (x, y) = (position.0 + dx, position.1 + dy);
        if walls.contains(&(x, y)) {
            continue;
        }
        if boxes.contains(&(x, y)) {
            let (mut xx, mut yy) = (x + dx, y + dy);
            while boxes.contains(&(xx, yy)) {
                (xx, yy) = (xx + dx, yy + dy);
            }
            if walls.contains(&(xx, yy)) {
                continue;
            }
            boxes.remove(&(x, y));
            boxes.insert((xx, yy));
        }
        position = (x, y);
    }

    let res = boxes.into_iter().map(|(x, y)| x * 100 + y).sum::<i32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("10092", process(input));
    }

    #[test]
    fn test_in2() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("2028", process(input));
    }
}
