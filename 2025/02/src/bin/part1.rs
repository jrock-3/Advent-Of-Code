use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let res = input
        .trim()
        .split(',')
        .map(|product| {
            let (start, end) = product.split_once('-').unwrap();
            let (start, end) = (start.parse::<u128>().unwrap(), end.parse::<u128>().unwrap());
            start..=end
        })
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|id| {
            let id_str = id.to_string();
            id_str.len() % 2 == 0 && id_str[..id_str.len() / 2].eq(&id_str[id_str.len() / 2..])
        })
        .sum::<u128>();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input));
    }
}
