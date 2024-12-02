use itertools::Itertools;

fn main() {
    let input = include_str!("../in/in.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    input
        .trim()
        .split("\n")
        .filter_map(|line| {
            let mut line = line
                .trim()
                .split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .tuple_windows()
                .peekable();

            let is_inc = line.peek().and_then(|(a, b)| Some(a < b)).unwrap();

            for (a, b) in line {
                let diff = (a - b).abs();

                let inc_to_dec = is_inc && a > b;
                let dec_to_inc = !is_inc && a < b;
                let out_of_bounds = diff > 3 || diff < 1;
                if inc_to_dec || dec_to_inc || out_of_bounds {
                    return None;
                }
            }
            Some(1)
        })
        .sum::<u32>()
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
