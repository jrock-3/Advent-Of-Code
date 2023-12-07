use std::collections::HashSet;

fn main() {
    let input = include_str!("../in/in1.txt");
    let res = process(input);
    dbg!("{}", res);
}

fn sum_set(set: &HashSet<char>) -> u32 {
    set.iter()
        .map(|c| {
            let num = u32::from(*c);
            if num >= u32::from('a') {
                num - u32::from('a') + 1
            } else {
                num - u32::from('A') + 27
            }
        })
        .sum::<u32>()
}

fn process(input: &str) -> String {
    let mut set = HashSet::new();
    let mut cnt = 0;
    let mut res = 0;

    for line in input.split('\n') {
        let x = line.chars().collect::<HashSet<_>>();
        if cnt == 0 {
            set = x;
        } else if cnt == 3 {
            res += sum_set(&set);
            set = x;
            cnt = 0;
        } else {
            set = set.intersection(&x).cloned().collect();
        }
        cnt += 1;
    }
    res += sum_set(&set);
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
        assert_eq!(res, "70");
    }
}
