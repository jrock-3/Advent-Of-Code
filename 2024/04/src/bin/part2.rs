use std::ops::Sub;

use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn is_xmas(arr: &Vec<Vec<char>>, i: usize, j: usize) -> Option<()> {
    let topleft = arr.get(i.checked_sub(1)?)?.get(j.checked_sub(1)?)?;
    let topright = arr.get(i.checked_sub(1)?)?.get(j.checked_add(1)?)?;
    let botleft = arr.get(i.checked_add(1)?)?.get(j.checked_sub(1)?)?;
    let botright = arr.get(i.checked_add(1)?)?.get(j.checked_add(1)?)?;

    match (topleft, topright, botleft, botright) {
        ('S', 'S', 'M', 'M')
        | ('M', 'S', 'M', 'S')
        | ('S', 'M', 'S', 'M')
        | ('M', 'M', 'S', 'S') => Some(()),
        _ => None,
    }
}

fn cnt_xmas(arr: &Vec<Vec<char>>) -> usize {
    let mut cnt = 0;

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j] == 'A' && is_xmas(arr, i, j).is_some() {
                cnt += 1;
            }
        }
    }

    cnt
}

fn process(input: &str) -> String {
    let word_search = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // dbg!(&word_search);

    cnt_xmas(&word_search).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input));
    }
}