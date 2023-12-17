fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

fn get_report(line: &str) -> Vec<i128> {
    line.split(' ')
        .map(|word| word.parse::<i128>().unwrap())
        .collect::<Vec<_>>()
}

fn get_reports(input: &str) -> Vec<Vec<i128>> {
    input.lines().map(get_report).collect::<Vec<_>>()
}

fn process_line(mut history: Vec<i128>) -> i128 {
    let mut cnt = 0;
    while history.as_slice()[0..history.len() - cnt]
        .iter()
        .any(|&num| num != 0)
    {
        for idx in 1..history.len() - cnt {
            history[idx - 1] = history[idx] - history[idx - 1];
        }
        cnt += 1;
    }

    if cnt >= history.len() - 1 {
        dbg!(cnt);
    }

    history.iter().skip(history.len() - cnt).sum::<i128>()
}

fn process(input: &str) -> String {
    let report = get_reports(input);

    let res = report.into_iter().map(process_line).collect::<Vec<_>>();
    dbg!(&res);
    dbg!(res.len());

    res.iter().sum::<i128>().to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("0 3 6 9 12 15", 18)]
    #[case("1 3 6 10 15 21", 28)]
    #[case("10 13 16 21 30 45", 68)]
    #[case("25 43 75 138 255 455 773 1250 1933 2875 4135 5778 7875 10503 13745 17690 22433 28075 34723 42490 51495", 61863)]
    #[case("25 47 77 111 155 235 402 724 1251 1932 2462 2054 -809 -8433 -23232 -45063 -64008 -43580 115154 646196 2060426", 5415590)]
    #[case("-10 -17 -26 -37 -50 -65 -82 -101 -122 -145 -170 -197 -226 -257 -290 -325 -362 -401 -442 -485 -530", -577)]
    #[case("8 14 15 5 -26 -94 -224 -454 -840 -1462 -2431 -3897 -6058 -9170 -13558 -19628 -27880 -38922 -53485 -72439 -96810", -127798)]
    fn line_test(#[case] line: &str, #[case] expected: i128) {
        let history = get_report(line);
        assert_eq!(process_line(history), expected)
    }

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("114", process(input));
    }
}
