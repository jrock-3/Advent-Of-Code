use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn process(input: &str) -> String {
    let heights = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // dbg!(&heights);

    (0..heights.len())
        .flat_map(|i| (0..heights[i].len()).map(move |j| (i as isize, j as isize)))
        .filter(|(i, j)| heights[*i as usize][*j as usize] == 0)
        .map(|(i, j)| {
            let mut cnt = 0;

            let mut queue = VecDeque::new();
            queue.push_back((i, j));

            // let mut seen = BTreeSet::new();

            while !queue.is_empty() {
                let (i, j) = queue.pop_back().unwrap();
                // dbg!((&i, &j));

                if heights[i as usize][j as usize] == 9 {
                    cnt += 1;
                    continue;
                }

                for (dx, dy) in DIRS {
                    let (new_i, new_j) = (i + dx, j + dy);
                    if (0..heights.len() as isize).contains(&new_i)
                        && (0..heights[0].len() as isize).contains(&new_j)
                        && heights[new_i as usize][new_j as usize]
                            == heights[i as usize][j as usize] + 1
                    {
                        queue.push_back((new_i, new_j));
                    }
                }
            }

            cnt
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input));
    }
}
