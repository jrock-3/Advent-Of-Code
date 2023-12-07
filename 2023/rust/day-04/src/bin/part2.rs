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

    let mut res = input
        .lines()
        .map(|line| {
            let (winning, mine) = line.split_once('|').unwrap();
            let win_set = winning
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<HashSet<_>>();
            let my = mine
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .filter(|num| win_set.contains(num))
                .count();
            (1, my)
        })
        .collect::<Vec<_>>();

    let mut idx = 0;
    while idx < res.len() {
        let (cnt, num) = res[idx];
        for i in idx + 1..res.len().min(idx + 1 + num) {
            res[i].0 += cnt;
        }
        idx += 1;
    }

    res.iter().map(|(cnt, _)| cnt).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("30", process(input));
    }
}
