fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let games = input.split("\n\n").map(|game| {
        let mut rules = game.lines();
        let btn_a = rules.next().unwrap().split_once(", ").unwrap();
        let btn_a = (
            btn_a.0[12..].parse::<isize>().unwrap(),
            btn_a.1[2..].parse::<isize>().unwrap(),
        );
        let btn_b = rules.next().unwrap().split_once(", ").unwrap();
        let btn_b = (
            btn_b.0[12..].parse::<isize>().unwrap(),
            btn_b.1[2..].parse::<isize>().unwrap(),
        );
        let prize = rules.next().unwrap().split_once(", ").unwrap();
        let prize = (
            prize.0[9..].parse::<isize>().unwrap(),
            prize.1[2..].parse::<isize>().unwrap(),
        );
        (btn_a, btn_b, prize)
    });

    games
        .filter_map(|((ax, ay), (bx, by), (px, py))| {
            (0..=100 as isize)
                .find(|a| {
                    (px - ax * a) % bx == 0
                        && (py - ay * a) % by == 0
                        && (px - ax * a) / bx == (py - ay * a) / by
                })
                .map(|a| a * 3 + (px - ax * a) / bx)
        })
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input));
    }
}
