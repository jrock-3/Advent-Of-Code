use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn process(input: &str) -> String {
    let plants = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    dbg!(&plants);

    // floodfill
    let mut cnt = 0;
    let mut visited = BTreeSet::new();
    for i in 0..plants.len() {
        for j in 0..plants[i].len() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let plant = plants[i][j];
            let mut stack = VecDeque::new();
            stack.push_back((i, j));

            let mut plant_cnt = 0;
            let mut perimeter = 0;
            // let mut seen_now = BTreeMap::new();
            while !stack.is_empty() {
                let (x, y) = stack.pop_back().unwrap();
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));
                let borders = DIRS
                    .iter()
                    .filter(|(dx, dy)| {
                        let (xx, yy) = (x as isize + dx, y as isize + dy);
                        !(0..plants.len() as isize).contains(&xx)
                            || !(0..plants[0].len() as isize).contains(&yy)
                            || plants[xx as usize][yy as usize] != plant
                    })
                    .count();
                // dbg!(&x, &y, &borders);
                // seen_now.insert((x, y), borders);
                perimeter += borders;
                plant_cnt += 1;

                for (dx, dy) in DIRS {
                    let (xx, yy) = (x as isize + dx, y as isize + dy);
                    if (0..plants.len() as isize).contains(&xx)
                        && (0..plants[0].len() as isize).contains(&yy)
                        && !visited.contains(&(xx as usize, yy as usize))
                        && plants[xx as usize][yy as usize] == plant
                    {
                        stack.push_back((xx as usize, yy as usize));
                    }
                }
            }

            // dbg!(seen_now);
            cnt += dbg!(plant_cnt) * dbg!(perimeter);
        }
    }
    // dbg!(&visited);

    cnt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input));
    }
}
