use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};
use itertools::Itertools;
use ndarray::prelude::*;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn cnt_xmas(arr: &Vec<Vec<&u32>>) -> usize {
    let xx = u32::from('X');
    let mm = u32::from('M');
    let aa = u32::from('A');
    let ss = u32::from('S');

    arr.iter()
        // .inspect(|row| {
        //     let chars = row
        //         .into_iter()
        //         .map(|&&c| char::from_u32(c).unwrap())
        //         .collect::<String>();
        //     dbg!(chars);
        // })
        .map(|row| {
            if row.len() < 4 {
                return 0;
            }
            row.iter()
                .tuple_windows()
                .filter(|(&&a, &&b, &&c, &&d)| {
                    a == xx && b == mm && c == aa && d == ss
                        || a == ss && b == aa && c == mm && d == xx
                })
                .count()
        })
        // .inspect(|row| {
        //     dbg!(&row);
        // })
        .sum::<usize>()
}

fn process(input: &str) -> String {
    let word_search = input
        .lines()
        .map(|line| line.chars().map(|c| u32::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Horizontal + Vertical
    let h_cnt = cnt_xmas(&straight_x(&word_search));
    dbg!(&h_cnt);
    // let h_cnt_rev = cnt_xmas(&straight_x(
    //     &word_search
    //         .clone()
    //         .into_iter()
    //         .map(|row| row.into_iter().rev().collect::<Vec<_>>())
    //         .collect::<Vec<_>>(),
    // ));
    // dbg!(&h_cnt_rev);
    let v_cnt = cnt_xmas(&straight_y(&word_search));
    dbg!(&v_cnt);
    // let v_cnt_rev = cnt_xmas(&straight_y(
    //     &word_search
    //         .clone()
    //         .into_iter()
    //         .map(|row| row.into_iter().rev().collect::<Vec<_>>())
    //         .collect::<Vec<_>>(),
    // ));
    // dbg!(&v_cnt_rev);
    let d_cnt = cnt_xmas(&diagonal_pos_pos(&word_search));
    dbg!(&d_cnt);
    // let d_cnt_rev = cnt_xmas(&diagonal_pos_pos(
    //     &word_search
    //         .clone()
    //         .into_iter()
    //         .map(|row| row.into_iter().rev().collect::<Vec<_>>())
    //         .collect::<Vec<_>>(),
    // ));
    // dbg!(&d_cnt_rev);
    let d_cnt2 = cnt_xmas(&diagonal_pos_neg(&word_search));
    dbg!(&d_cnt2);
    // let d_cnt2_rev = cnt_xmas(&diagonal_pos_neg(
    //     &word_search
    //         .clone()
    //         .into_iter()
    //         .map(|row| row.into_iter().rev().collect::<Vec<_>>())
    //         .collect::<Vec<_>>(),
    // ));
    // dbg!(&d_cnt2_rev);

    (h_cnt + v_cnt + d_cnt + d_cnt2).to_string()
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
        assert_eq!("18", process(input));
    }
}
