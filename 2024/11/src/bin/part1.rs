fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let mut nums = input
        .trim()
        .split(" ")
        .map(|num| num.to_string())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        nums = nums
            .into_iter()
            .flat_map(|num| {
                if num == "0".to_string() {
                    return vec!["1".to_string()].into_iter();
                }
                if num.len() % 2 == 0 {
                    return vec![
                        num[0..num.len() / 2].parse::<usize>().unwrap().to_string(),
                        num[num.len() / 2..].parse::<usize>().unwrap().to_string(),
                    ]
                    .into_iter();
                }
                return vec![(num.parse::<usize>().unwrap() * 2024).to_string()].into_iter();
            })
            .collect::<Vec<_>>();
    }

    nums.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "125 17";
        assert_eq!("55312", process(input));
    }
}
