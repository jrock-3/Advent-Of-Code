fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let (turns, _) = input
        .lines()
        .map(|cmd| {
            let dir = match cmd.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => unreachable!(),
            };
            dir * cmd[1..].parse::<i32>().unwrap()
        })
        .fold((0, 50), |(mut cnt, idx), dir| {
            let new_idx = (idx + dir) % 100;
            if new_idx == 0 {
                cnt += 1;
            }
            (cnt, new_idx)
        });
    turns.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process(input));
    }
}
