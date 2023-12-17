use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug, PartialEq)]
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
) -> usize {
    let mut curr = start;
    let mut prev = previous;
    let mut cnt = 0;
    loop {
        (prev, curr) = (curr, find_next(curr, prev, neighbors));
        cnt += 1;
        if curr == previous {
            break;
        }
    }
    cnt
}

fn process(input: &str) -> String {
    let neighbors = generate_neighbors(input);
    let start = find_start(input);

    let mut paths = vec![];
    for dir in neighbors[&start].iter() {
        let opposite = match dir {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        };
        let neighbor = to_neighbor(start, dir);
        if neighbors.contains_key(&neighbor) && neighbors[&neighbor].contains(&opposite) {
            paths.push(find_cycle(neighbor, start, &neighbors));
        }
    }

    let res = paths.iter().min().unwrap() / 2 + 1;

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_in2() {
        let input = include_str!("../in/sample2.txt");
        assert_eq!("8", process(input));
    }
}
