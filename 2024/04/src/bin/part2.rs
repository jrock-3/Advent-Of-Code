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

fn process(input: &str) -> String {
    let word_search = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // dbg!(&word_search);

    (0..word_search.len())
        .cartesian_product(0..word_search[0].len())
        .filter(|&(i, j)| word_search[i][j] == 'A' && is_xmas(&word_search, i, j).is_some())
        .count()
        .to_string()
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
