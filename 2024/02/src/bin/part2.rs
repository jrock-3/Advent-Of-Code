use itertools::Itertools;

fn main() {
    let input = include_str!("../in/in.txt");
    dbg!(process(input));
}

fn is_valid(is_inc: bool, a: i32, b: i32) -> bool {
    let diff = (a - b).abs();

    let inc_to_dec = is_inc && a > b;
    let dec_to_inc = !is_inc && a < b;
    let out_of_bounds = diff > 3 || diff < 1;
    return !(inc_to_dec || dec_to_inc || out_of_bounds);
}

fn is_line_valid(line: &Vec<i32>) -> bool {
    let is_inc = line[0] < line[1];

    for (&a, &b) in line.into_iter().tuple_windows() {
        if !is_valid(is_inc, a, b) {
            return false;
        }
    }

    true
}

fn process(input: &str) -> String {
    let lines = input.trim().split("\n").map(|line| {
        line.trim()
            .split(" ")
            .map(|level| level.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut cnt = 0;
    for mut line in lines {
        // dbg!(&line);
        if is_line_valid(&line) {
            // dbg!(&line);
            cnt += 1;
            continue;
        }

        // try removing a level
        for i in 0..line.len() {
            // dbg!(&line);
            let lvl = line.remove(i);
            if is_line_valid(&line) {
                cnt += 1;
                break;
            }
            line.insert(i, lvl);
        }
    }

    cnt.to_string()
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
