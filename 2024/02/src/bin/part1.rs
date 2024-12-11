use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    input
        .trim()
        .split("\n")
        .filter(|line| {
            let mut line = line
                .trim()
                .split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .tuple_windows()
                .peekable();

            let is_inc = line.peek().and_then(|(a, b)| Some(a < b)).unwrap();

            line.all(|(a, b)| {
                (1..=3).contains(&(a - b).abs()) && (if is_inc { a < b } else { a > b })
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
        assert_eq!("2", process(input));
    }
}
