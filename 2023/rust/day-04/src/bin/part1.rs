use std::collections::HashSet;

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    // for each line:
    // split winning and losing
    // count number of matching in winning & losing -> n
    // bitshift by n (0 if n == 0)
    // add up everything

    let res = input
        .lines()
        .map(|line| {
            let (winning, mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let win_set = winning
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<HashSet<_>>();
            let my = mine
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .filter(|num| win_set.contains(num))
                .count();
            if my == 0 {
                0
            } else {
                1 << (my - 1)
            }
        })
        .sum::<u32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("13", process(input));
    }
}
