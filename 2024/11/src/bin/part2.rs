use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let mut nums = input
        .trim()
        .split(" ")
        .map(|num| num.parse::<usize>().unwrap())
        .counts();

    for _ in 0..75 {
        for (num, cnt) in nums.clone() {
            *nums.get_mut(&num).unwrap() -= cnt;

            match num {
                0 => {
                    nums.entry(1)
                        .and_modify(|e| {
                            *e += cnt;
                        })
                        .or_insert(cnt);
                }
                num if num.ilog10() % 2 == 1 => {
                    let digits = 10usize.pow((num.ilog10() + 1) / 2);
                    nums.entry(num / digits)
                        .and_modify(|e| {
                            *e += cnt;
                        })
                        .or_insert(cnt);
                    nums.entry(num % digits)
                        .and_modify(|e| {
                            *e += cnt;
                        })
                        .or_insert(cnt);
                }
                _ => {
                    nums.entry(num * 2024)
                        .and_modify(|e| {
                            *e += cnt;
                        })
                        .or_insert(cnt);
                }
            }
        }
    }

    nums.values().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "125 17";
        assert_eq!("", process(input));
    }
}
