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
    lbox: &BTreeSet<(i32, i32)>,
    rbox: &BTreeSet<(i32, i32)>,
) {
    println!(
        "{}\n",
        (0..xlen as i32)
            .map(|i| {
                let walls = walls.clone();
                let lbox = lbox.clone();
                let rbox = rbox.clone();
                (0..ylen as i32)
                    .map(move |j| {
                        if walls.contains(&(i, j)) {
                            '#'
                        } else if lbox.contains(&(i, j)) {
                            '['
                        } else if rbox.contains(&(i, j)) {
                            ']'
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
    let ylen = input.lines().next().unwrap().len() * 2;
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = map.lines().map(|line| line.chars());

    let mut walls = BTreeSet::new();
    let mut lbox = BTreeSet::new();
    let mut rbox = BTreeSet::new();
    let mut position = (-1, -1);

    for (i, row) in map.enumerate() {
        let i = i as i32;
        for (j, dir) in row.enumerate() {
            let j = j as i32;
            match dir {
                '#' => {
                    walls.insert((i, 2 * j));
                    walls.insert((i, 2 * j + 1));
                }
                'O' => {
                    lbox.insert((i, 2 * j));
                    rbox.insert((i, 2 * j + 1));
                }
                '@' => {
                    position = (i, 2 * j);
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    // print_map(xlen, ylen, &walls, &boxes);

    for dir in moves.chars().filter(|&c| c != '\n') {
        print_map(xlen, ylen, position, &walls, &lbox, &rbox);
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
        // FIXME: handle 2 wide boxes
        if lbox.contains(&(x, y)) {
            // left
            let (mut xx, mut yy) = (x + dx, y + dy);
            while lbox.contains(&(xx, yy)) {
                (xx, yy) = (xx + dx, yy + dy);
            }
            if walls.contains(&(xx, yy)) {
                continue;
            }
            lbox.remove(&(x, y));
            lbox.insert((xx, yy));
        }
        position = (x, y);
    }

    let res = lbox.into_iter().map(|(x, y)| x * 100 + y).sum::<i32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
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
        assert_eq!("9021", process(input));
    }

    #[test]
    fn test_in2() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq!("", process(input));
    }
}
