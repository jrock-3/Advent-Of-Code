use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../in/in1.txt");
    let res = process(input);
    dbg!("{}", res);
}

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn process(input: &str) -> String {
    let re = Regex::new(r"\d+|[^\w\d\.]").unwrap();

    let mut num_coord_pair = Vec::new();
    let mut parts = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for hit in re.find_iter(line) {
            match hit.as_str().parse::<i32>() {
                Ok(n) => {
                    num_coord_pair.push((
                        n,
                        hit.range()
                            .map(|col| (row as i32, col as i32))
                            .collect::<HashSet<_>>(),
                    ));
                }
                Err(_) => {
                    parts.insert((row as i32, hit.start() as i32));
                }
            }
        }
    }

    // IDEA FROM ADVENTOFCODE REDDIT: iterate through numbers instead of parts for cleaner code + removal of "seen"
    let mut total = 0;
    for (row, col) in parts {
        let mut seen = HashSet::new();
        for (dr, dc) in DIRS {
            let coord = (row + dr, col + dc);
            if let Some((n, _)) = num_coord_pair
                .iter()
                .find(|(_, set)| set.contains(&coord) && seen.intersection(set).count() == 0)
            {
                total += n;
                seen.insert(coord);
            }
        }
    }

    // dbg!(rev_num_coords);
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        let res = process(input);
        assert_eq!(res, "4361");
    }
}
