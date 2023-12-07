use std::collections::HashSet;

fn main() {
    let input = include_str!("../in/in1.txt");
    let res = process(input);
    dbg!("{}", res);
}

fn process(input: &str) -> String {
    let res = input
        .split('\n')
        .map(|line| {
            // split in half
            let first = line[0..line.len() / 2].chars().collect::<HashSet<_>>();
            let second = line[line.len() / 2..line.len()]
                .chars()
                .collect::<HashSet<_>>();

            // set intersection
            first
                .intersection(&second)
                .map(|c| {
                    let num = u32::from(*c);
                    // subtract from 'a' to get value
                    if num >= u32::from('a') {
                        num - u32::from('a') + 1
                    } else {
                        num - u32::from('A') + 27
                    }
                })
                .sum::<u32>()
        })
        //     .collect::<Vec<_>>();
        // dbg!("{}", res);
        .sum::<u32>();
    res.to_string()
    // input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        let res = process(input);
        assert_eq!(res, "157");
    }
}
