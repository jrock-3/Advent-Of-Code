use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;

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

    // NOTE: Taken from [Wikipedia Dijkstra's Algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm#Pseudocode)
    let mut pq = tiles
        .iter()
        .flat_map(|&pos| DIRS.iter().map(move |&(dir, _)| (dir, pos)))
        .collect::<BTreeSet<_>>();

    let mut dist = pq
        .iter()
        .map(|&state| (state, i32::MAX))
        .collect::<BTreeMap<_, _>>();
    *dist.get_mut(&(Dir::East, start)).unwrap() = 0;

    let mut prev = BTreeMap::new();

    while !pq.is_empty() {
        let (dir, tile) = {
            let tile = pq
                .iter()
                .min_by_key(|&state| dist.get(state))
                .unwrap()
                .clone();
            pq.remove(&tile);
            tile
        };
        if tile == end {
            break;
        }

        // NOTE: Taken from [Dijkstra's Algorithm Find All Shortest Paths Possible](https://stackoverflow.com/questions/2819347/dijkstras-algorithm-to-find-all-the-shortest-paths-possible)
        for (new_dir, new_tile, cost) in DIRS
            .iter()
            .map(|&(pos, (dx, dy))| (pos, (tile.0 + dx, tile.1 + dy), 1))
            .filter(|(_, tile, _)| tiles.contains(&tile))
            .chain(DIRS.iter().map(|&(dir, _)| (dir, tile, 1000)))
            .filter(|&(new_dir, new_tile, _)| dir == new_dir || tile == new_tile)
            .filter(|&(new_dir, new_tile, _)| !(dir == new_dir && tile == new_tile))
        {
            let new_dist = dist[&(dir, tile)] + cost;

            if new_dist <= dist[&(new_dir, new_tile)] {
                *dist.get_mut(&(new_dir, new_tile)).unwrap() = new_dist;

                if new_dist < dist[&(new_dir, new_tile)] {
                    prev.insert((new_dir, new_tile), {
                        let mut set = BTreeSet::new();
                        set.insert((dir, tile));
                        set
                    });
                } else {
                    prev.entry((new_dir, new_tile))
                        .and_modify(|set| {
                            set.insert((dir, tile));
                        })
                        .or_insert({
                            let mut set = BTreeSet::new();
                            set.insert((dir, tile));
                            set
                        });
                }
            }
        }
    }

    let mut res = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(end);
    while !queue.is_empty() {
        let curr = queue.pop_back().unwrap();
        if res.contains(&curr) {
            continue;
        }
        res.insert(curr);

        // let smallest = DIRS
        //     .iter()
        //     .map(|&(dir, _)| dist[&(dir, curr)])
        //     .min()
        //     .unwrap();

        for set in DIRS
            .iter()
            // .filter(|&&(dir, _)| dist[&(dir, curr)] == smallest)
            .filter_map(|&(dir, _)| prev.get(&(dir, curr)))
        {
            queue.extend(set.iter().map(|&(_, tile)| tile));
        }
    }

    println!(
        "{}",
        (0..input.lines().collect::<Vec<_>>().len() as i32)
            .map(|i| {
                let res = res.clone();
                let walls = walls.clone();
                (0..input.lines().next().unwrap().len() as i32)
                    .map(move |j| {
                        if res.contains(&(i, j)) {
                            'O'
                        } else if walls.contains(&(i, j)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .join("\n")
    );

    res.len().to_string()
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
        assert_eq!("45", process(input));
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
        assert_eq!("64", process(input));
    }
}
