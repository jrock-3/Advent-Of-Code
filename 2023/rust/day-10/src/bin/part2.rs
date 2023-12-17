use std::collections::{BTreeMap, BTreeSet};

use rust_fsm::{state_machine, StateMachine};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn generate_neighbors(input: &str) -> BTreeMap<(isize, isize), Vec<Dir>> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c == '.' {
                    return None;
                }
                Some((
                    (row as isize, col as isize),
                    match c {
                        'S' => vec![Dir::Left, Dir::Right, Dir::Up, Dir::Down],
                        '|' => vec![Dir::Up, Dir::Down],
                        '-' => vec![Dir::Left, Dir::Right],
                        'F' => vec![Dir::Right, Dir::Down],
                        '7' => vec![Dir::Left, Dir::Down],
                        'J' => vec![Dir::Left, Dir::Up],
                        'L' => vec![Dir::Right, Dir::Up],
                        _ => unreachable!(),
                    },
                ))
            })
        })
        .collect::<BTreeMap<_, _>>()
}

fn find_start(input: &str) -> (isize, isize) {
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                return (row as isize, col as isize);
            }
        }
    }
    unreachable!()
}

fn to_neighbor(curr: (isize, isize), dir: &Dir) -> (isize, isize) {
    match dir {
        Dir::Left => (curr.0, curr.1 - 1),
        Dir::Right => (curr.0, curr.1 + 1),
        Dir::Up => (curr.0 - 1, curr.1),
        Dir::Down => (curr.0 + 1, curr.1),
    }
}

fn find_next(
    curr: (isize, isize),
    prev: (isize, isize),
    neighbors: &BTreeMap<(isize, isize), Vec<Dir>>,
) -> (isize, isize) {
    for dir in neighbors[&curr].iter() {
        let neighbor = to_neighbor(curr, dir);
        if neighbor != prev && neighbors.contains_key(&neighbor) {
            return neighbor;
        }
    }
    unreachable!()
}

fn find_cycle(
    start: (isize, isize),
    previous: (isize, isize),
    neighbors: &BTreeMap<(isize, isize), Vec<Dir>>,
) -> BTreeSet<(isize, isize)> {
    let mut curr = start;
    let mut prev = previous;
    let mut res = BTreeSet::from([curr]);
    loop {
        (prev, curr) = (curr, find_next(curr, prev, neighbors));
        res.insert(curr);
        if curr == previous {
            break;
        }
    }
    res
}

fn get_cycle(
    start: (isize, isize),
    neighbors: &BTreeMap<(isize, isize), Vec<Dir>>,
) -> (BTreeSet<(isize, isize)>, char) {
    let mut res = BTreeSet::new();
    let mut dirs = vec![];
    for dir in neighbors[&start].iter() {
        let opposite = match dir {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        };
        let neighbor = to_neighbor(start, dir);
        if neighbors.contains_key(&neighbor) && neighbors[&neighbor].contains(&opposite) {
            res = find_cycle(neighbor, start, neighbors);
            dirs.push(dir);
        }
    }
    dirs.sort();
    dbg!(&dirs);
    let c = match dirs.as_slice() {
        [Dir::Left, Dir::Right] => '-',
        [Dir::Left, Dir::Up] => 'J',
        [Dir::Left, Dir::Down] => '7',
        [Dir::Right, Dir::Up] => 'L',
        [Dir::Right, Dir::Down] => 'F',
        [Dir::Up, Dir::Down] => '|',
        _ => unreachable!(),
    };
    (res, c)
}

state_machine! {
    derive(Debug)
    PointLoc(Outside)

    Outside => {
        L => OutsideUp,
        F => OutsideDown,
        Pipe => Inside,
    },
    OutsideUp => {
        Dash => OutsideUp,
        J => Outside,
        Seven => Inside,
    },
    OutsideDown => {
        Dash => OutsideDown,
        Seven => Outside,
        J => Inside,
    },
    Inside => {
        L => InsideUp,
        F => InsideDown,
        Pipe => Outside,
    },
    InsideUp => {
        Dash => InsideUp,
        J => Inside,
        Seven => Outside,
    },
    InsideDown => {
        Dash => InsideDown,
        Seven => Inside,
        J => Outside,
    },
}

fn get_inside_count(input: &str, start_translated: char, cycle: &BTreeSet<(isize, isize)>) -> u128 {
    // here, we have what is the cycle and what is not the cycle
    // the next step is to determine how something is enclosed in a loop
    // approach 1: from the origin, pass through odd numbered = inside
    // origin or (0, R) to (C, R)

    // issue: how to tell if a line (LJ or L7 or any variants) is "crossing" or not
    // idea: some sort of state machine
    // ie outside -> see L -> see N x - -> see 7 -> inside
    // vs outside -> see L -> see N x - -> see J -> outside

    // simplifications:
    // - only look in curr row (don't look at other rows)
    // - always scan left to right (0 to COL)

    let mut cnt = 0;
    let mut machine: StateMachine<PointLoc> = StateMachine::new();

    for (row, line) in input.lines().enumerate() {
        for (col, mut c) in line.chars().enumerate() {
            if cycle.contains(&(row as isize, col as isize)) {
                if c == 'S' {
                    c = start_translated;
                }
                match c {
                    '|' => machine.consume(&PointLocInput::Pipe).unwrap(),
                    '-' => machine.consume(&PointLocInput::Dash).unwrap(),
                    'F' => machine.consume(&PointLocInput::F).unwrap(),
                    '7' => machine.consume(&PointLocInput::Seven).unwrap(),
                    'J' => machine.consume(&PointLocInput::J).unwrap(),
                    'L' => machine.consume(&PointLocInput::L).unwrap(),
                    _ => unreachable!(),
                };
            } else {
                match machine.state() {
                    PointLocState::Inside => cnt += 1,
                    PointLocState::Outside => (),
                    PointLocState::OutsideDown
                    | PointLocState::OutsideUp
                    | PointLocState::InsideDown
                    | PointLocState::InsideUp => unreachable!(),
                }
            }
        }
    }

    cnt
}

fn process(input: &str) -> String {
    let neighbors = generate_neighbors(input);
    let start = find_start(input);

    let (cycle, start_translated) = get_cycle(start, &neighbors);

    let inside_tiles = get_inside_count(input, start_translated, &cycle);

    inside_tiles.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in3() {
        let input = include_str!("../in/sample3.txt");
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_in4() {
        let input = include_str!("../in/sample4.txt");
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_in5() {
        let input = include_str!("../in/sample5.txt");
        assert_eq!("8", process(input));
    }

    #[test]
    fn test_in6() {
        let input = include_str!("../in/sample6.txt");
        assert_eq!("10", process(input));
    }
}
