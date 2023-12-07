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
    let re = Regex::new(r"\d+|\*").unwrap();

    let mut num_coord_pair = Vec::new();
    let mut potential_gears = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for hit in re.find_iter(line) {
            match hit.as_str().parse::<i32>() {
                Ok(n) => {
                    let mut coords = HashSet::new();
                    for col in hit.range() {
                        coords.insert((row as i32, col as i32));
                    }
                    num_coord_pair.push((n, coords));
                }
                Err(_) => {
                    potential_gears.insert((row as i32, hit.start() as i32));
                }
            }
        }
    }

    let mut total = 0;
    for (row, col) in potential_gears {
        let mut seen = HashSet::new();
        let mut cnt = 0;
        let mut ratio = 1;
        for (dr, dc) in DIRS {
            let coord = (row + dr, col + dc);
            if let Some((n, _)) = num_coord_pair
                .iter()
                .find(|(_, set)| set.contains(&coord) && seen.intersection(set).count() == 0)
            {
                if cnt == 2 {
                    break;
                }
                cnt += 1;
                ratio *= n;
                seen.insert(coord);
            }
        }
        if cnt == 2 {
            total += ratio;
        }
    }

    // dbg!(num_coord_pair);
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        let res = process(input);
        assert_eq!(res, "467835");
    }
}
