fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

fn get_report(line: &str) -> Vec<i128> {
    line.split(' ')
        .map(|word| word.parse::<i128>().unwrap())
        .rev()
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

    history.iter().skip(history.len() - cnt).sum::<i128>()
}

fn process(input: &str) -> String {
    let report = get_reports(input);

    let res = report.into_iter().map(process_line).collect::<Vec<_>>();

    res.iter().sum::<i128>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("2", process(input));
    }
}
