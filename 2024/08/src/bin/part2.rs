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
) -> impl Iterator<Item = (usize, usize)> {
    let xdiff = x1 - x2;
    let ydiff = y1 - y2;

    vec![
        (1..)
            .map(|i| (x1 + i * xdiff, y1 + i * ydiff))
            .take_while(|(x, y)| (0..xlen).contains(&x) && (0..ylen).contains(&y))
            .collect::<Vec<_>>(),
        (1..)
            .map(|i| (x1 - i * xdiff, y1 - i * ydiff))
            .take_while(|(x, y)| (0..xlen).contains(&x) && (0..ylen).contains(&y))
            .collect::<Vec<_>>(),
        (1..)
            .map(|i| (x2 + i * xdiff, y2 + i * ydiff))
            .take_while(|(x, y)| (0..xlen).contains(&x) && (0..ylen).contains(&y))
            .collect::<Vec<_>>(),
        (1..)
            .map(|i| (x2 - i * xdiff, y2 - i * ydiff))
            .take_while(|(x, y)| (0..xlen).contains(&x) && (0..ylen).contains(&y))
            .collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
    .map(|(x, y)| (usize::try_from(x).unwrap(), usize::try_from(y).unwrap()))
}

fn process(input: &str) -> String {
    let xlen = input.lines().count();
    let ylen = input.lines().next().unwrap().len();
    // dbg!(&xlen, &ylen);

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
    dbg!(&antennas);

    let mut antinodes = BTreeSet::new();
    antennas.values().for_each(|locs| {
        dbg!(&locs);
        locs.iter()
            .tuple_combinations()
            .for_each(|(&(x1, y1), &(x2, y2))| {
                for node in get_nodes(
                    (x1.try_into().unwrap(), y1.try_into().unwrap()),
                    (x2.try_into().unwrap(), y2.try_into().unwrap()),
                    xlen.try_into().unwrap(),
                    ylen.try_into().unwrap(),
                ) {
                    dbg!(&node);
                    antinodes.insert(node);
                }
            })
    });
    // dbg!(&antinodes);

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
        assert_eq!("34", process(input));
    }
}
