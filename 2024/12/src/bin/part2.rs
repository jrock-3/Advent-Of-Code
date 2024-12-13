// NOTE: Got [hint](https://www.reddit.com/r/adventofcode/comments/1hcpyic/2024_day_12_part_2_what_kind_of_algorithm_did_you/) # corners = # walls
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const DIAG_DIRS: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

fn process(input: &str) -> String {
    let plants = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as isize, j as isize), c))
        })
        .collect::<BTreeMap<_, _>>();
    let xlen = *plants.keys().map(|(i, _)| i).max().unwrap();
    let ylen = *plants.keys().map(|(_, j)| j).max().unwrap();

    let mut res = 0;
    let mut visited = BTreeSet::new();
    (0..=xlen).cartesian_product(0..=ylen).for_each(|(i, j)| {
        if visited.contains(&(i, j)) {
            return;
        }

        let plant = plants[&(i, j)];
        let mut stack = VecDeque::new();
        stack.push_back((i, j));

        let mut cnt = 0;
        let mut area = 0;
        while !stack.is_empty() {
            let (x, y) = stack.pop_back().unwrap();
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            area += 1;
            for (dx, dy) in DIAG_DIRS {
                if let [_, false, false] | [false, true, true] =
                    [(x + dx, y + dy), (x, y + dy), (x + dx, y)]
                        .iter()
                        .map(|point| plants.get(point).is_some_and(|&p| p == plant))
                        .collect::<Vec<_>>()[..]
                {
                    cnt += 1;
                }
            }

            stack.extend(
                DIRS.iter()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|&(xx, yy)| {
                        !visited.contains(&(xx, yy))
                            && plants.get(&(xx, yy)).is_some_and(|&p| p == plant)
                    }),
            );
        }
        // dbg!(&cnt, &area);
        res += cnt * area;
    });

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input));
    }

    #[test]
    fn test_in2() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("80", process(input));
    }

    #[test]
    fn test_in3() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("436", process(input));
    }

    #[test]
    fn test_in4() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!("236", process(input));
    }

    #[test]
    fn test_in5() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!("368", process(input));
    }
}
