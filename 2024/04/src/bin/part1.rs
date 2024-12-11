use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};
use itertools::Itertools;

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
        .filter(|row| row.len() >= 4)
        .map(|row| {
            row.iter()
                .tuple_windows()
                .filter(|(&&a, &&b, &&c, &&d)| {
                    a == xx && b == mm && c == aa && d == ss
                        || a == ss && b == aa && c == mm && d == xx
                })
                .count()
        })
        .sum::<usize>()
}

fn process(input: &str) -> String {
    let word_search = input
        .lines()
        .map(|line| line.chars().map(|c| u32::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Horizontal + Vertical
    let h_cnt = cnt_xmas(&straight_x(&word_search));
    let v_cnt = cnt_xmas(&straight_y(&word_search));
    let d_cnt = cnt_xmas(&diagonal_pos_pos(&word_search));
    let d_cnt2 = cnt_xmas(&diagonal_pos_neg(&word_search));

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
