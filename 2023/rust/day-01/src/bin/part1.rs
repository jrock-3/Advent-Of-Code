fn main() {
    let input = include_str!("../in/in1.txt");
    let res = process(input);
    dbg!("{:?}", res);
}

fn process(input: &str) -> String {
    let res = input
        .split('\n')
        .map(|line| {
            (line
                .chars()
                .nth(line.find(char::is_numeric).unwrap())
                .unwrap()
                .to_string()
                + line
                    .chars()
                    .rev()
                    .nth(
                        line.chars()
                            .rev()
                            .collect::<String>()
                            .find(char::is_numeric)
                            .unwrap(),
                    )
                    .unwrap()
                    .to_string()
                    .as_str())
            .parse::<u32>()
            .unwrap()
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
        let res = process(input);
        assert_eq!(res, "142")
    }
}
