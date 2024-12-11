use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn is_line_valid(line: &Vec<i32>) -> bool {
    let is_inc = line[0] < line[1];

    line.into_iter()
        .tuple_windows()
        .all(|(&a, &b)| (1..=3).contains(&(a - b).abs()) && (if is_inc { a < b } else { a > b }))
}

fn process(input: &str) -> String {
    let lines = input.trim().split("\n").map(|line| {
        line.trim()
            .split(" ")
            .map(|level| level.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    lines
        .filter(|line| {
            is_line_valid(&line)
                || (0..line.len()).any(|i| {
                    let mut line = line.clone();
                    line.remove(i);
                    is_line_valid(&line)
                })
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input));
    }
}
