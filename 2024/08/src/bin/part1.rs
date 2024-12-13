use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn get_nodes(
    (x1, y1): (isize, isize),
    (x2, y2): (isize, isize),
    xlen: isize,
    ylen: isize,
) -> Vec<(usize, usize)> {
    let xdiff = x1 - x2;
    let ydiff = y1 - y2;

    vec![
        (x1 + xdiff, y1 + ydiff),
        (x1 - xdiff, y1 - ydiff),
        (x2 + xdiff, y2 + ydiff),
        (x2 - xdiff, y2 - ydiff),
    ]
    .into_iter()
    .filter(|&(x, y)| {
        (x, y) != (x1, y1) && (x, y) != (x2, y2) && (0..xlen).contains(&x) && (0..ylen).contains(&y)
    })
    .map(|(x, y)| (x.try_into().unwrap(), y.try_into().unwrap()))
    .collect()
}

fn process(input: &str) -> String {
    let xlen = input.lines().count();
    let ylen = input.lines().next().unwrap().len();

    let mut antennas = BTreeMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| c != '.')
            .for_each(|(j, c)| {
                antennas
                    .entry(c)
                    .and_modify(|set: &mut BTreeSet<(usize, usize)>| {
                        set.insert((i, j));
                    })
                    .or_insert_with(|| {
                        let mut set = BTreeSet::new();
                        set.insert((i, j));
                        set
                    });
            });
    });

    let mut antinodes = BTreeSet::new();
    antennas.values().for_each(|locs| {
        locs.iter()
            .tuple_combinations()
            .for_each(|(&(x1, y1), &(x2, y2))| {
                for node in get_nodes(
                    (x1.try_into().unwrap(), y1.try_into().unwrap()),
                    (x2.try_into().unwrap(), y2.try_into().unwrap()),
                    xlen.try_into().unwrap(),
                    ylen.try_into().unwrap(),
                ) {
                    antinodes.insert(node);
                }
            })
    });

    antinodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input));
    }
}
