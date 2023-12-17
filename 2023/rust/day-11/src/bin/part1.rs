use std::collections::BTreeSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

fn get_empty(map: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut cols = vec![0; u16::MAX as usize];
    let mut rows = vec![0; u16::MAX as usize];
    for (i, row) in map.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            rows[i] = 1;
        }
    }
    for i in 0..map[0].len() {
        let mut trip = false;
        for row in map {
            if row[i] == '#' {
                trip = true;
                break;
            }
        }
        if !trip {
            cols[i] = 1;
        }
    }

    (cols, rows)
}

fn process(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (cols, rows) = get_empty(&map);

    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c != '#' {
                    return None;
                }
                Some((row, col))
            })
        })
        .collect::<BTreeSet<_>>();

    let res = galaxies
        .into_iter()
        .map(|(row, col)| {
            (
                row + rows.as_slice()[..row].iter().sum::<usize>(),
                col + cols.as_slice()[..col].iter().sum::<usize>(),
            )
        })
        .collect::<Vec<_>>()
        .into_iter()
        .tuple_combinations::<(_, _)>()
        .map(|((r1, c1), (r2, c2))| r1.abs_diff(r2) + c1.abs_diff(c2))
        .sum::<usize>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("374", process(input));
    }
}
