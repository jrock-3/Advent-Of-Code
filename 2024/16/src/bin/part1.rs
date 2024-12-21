use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Dir {
    North,
    South,
    East,
    West,
}

const DIRS: [(Dir, (i32, i32)); 4] = [
    (Dir::North, (-1, 0)),
    (Dir::South, (1, 0)),
    (Dir::East, (0, 1)),
    (Dir::West, (0, -1)),
];

fn process(input: &str) -> String {
    let mut tiles = BTreeSet::new();
    let mut walls = BTreeSet::new();
    let mut start = (-1, -1);
    let mut end = (-1, -1);
    for (i, line) in input.lines().enumerate() {
        let i = i as i32;
        for (j, tile) in line.chars().enumerate() {
            let j = j as i32;
            match tile {
                '.' => {
                    tiles.insert((i, j));
                }
                '#' => {
                    walls.insert((i, j));
                }
                'S' => {
                    start = (i, j);
                    tiles.insert((i, j));
                }
                'E' => {
                    end = (i, j);
                    tiles.insert((i, j));
                }
                _ => unreachable!(),
            }
        }
    }
    // dbg!(&tiles, &walls, start, end);

    let mut pq = tiles
        .iter()
        .flat_map(|&pos| {
            [
                (Dir::North, pos),
                (Dir::South, pos),
                (Dir::East, pos),
                (Dir::West, pos),
            ]
        })
        .collect::<Vec<_>>();

    let mut dist = pq
        .iter()
        .map(|&state| (state, i32::MAX))
        .collect::<BTreeMap<_, _>>();
    *dist.get_mut(&(Dir::East, start)).unwrap() = 0;

    let mut prev: BTreeMap<(Dir, (i32, i32)), (Dir, (i32, i32))> = BTreeMap::new();

    while !pq.is_empty() {
        let (dir, tile) = {
            let (idx, _) = pq
                .iter()
                .enumerate()
                .min_by_key(|(_, state)| dist.get(state))
                .unwrap();
            pq.remove(idx)
        };
        if tile == end {
            break;
        }

        for (new_dir, new_tile, cost) in DIRS
            .iter()
            .map(|&(pos, (dx, dy))| (pos, (tile.0 + dx, tile.1 + dy), 1))
            .filter(|(_, tile, _)| tiles.contains(&tile))
            .chain(DIRS.iter().map(|&(dir, _)| (dir, tile, 1000)))
            .filter(|&(new_dir, new_tile, _)| dir == new_dir || tile == new_tile)
            .filter(|&(new_dir, new_tile, _)| !(dir == new_dir && tile == new_tile))
        {
            // dbg!((&new_dir, &new_tile, &cost));
            let new_dist = dist[&(dir, tile)] + cost;
            // dbg!(new_dist, dist[&(new_dir, new_tile)]);
            if new_dist < dist[&(new_dir, new_tile)] {
                *dist.get_mut(&(new_dir, new_tile)).unwrap() = new_dist;
                prev.insert((new_dir, new_tile), (dir, tile));
            }
        }
        // dbg!(&prev);
        // dbg!(dist
        //     .iter()
        //     .filter(|&(_, &val)| val < i32::MAX)
        //     .collect::<Vec<_>>());
    }

    let res = DIRS
        .iter()
        .map(|&(dir, _)| dist[&(dir, end)])
        .min()
        .unwrap();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("7036", process(input));
    }

    #[test]
    fn test_in2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("11048", process(input));
    }
}
