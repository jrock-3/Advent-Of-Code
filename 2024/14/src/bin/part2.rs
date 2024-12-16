use std::{
    collections::BTreeSet,
    io::{stdin, stdout, Write},
    thread, time,
};

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() -> miette::Result<()> {
    let input = include_str!("../input.txt");
    dbg!(process(input, 101, 103)?);

    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight,
}

type Pos = (i32, i32);
#[derive(Debug)]
struct Robot {
    position: Pos,
    vel: Pos,
}

impl Robot {
    fn simulate(mut self, width: i32, height: i32, iters: i32) -> Self {
        self.position.0 =
            (self.position.0 + (self.vel.0 * iters).rem_euclid(width)).rem_euclid(width);
        self.position.1 =
            (self.position.1 + (self.vel.1 * iters).rem_euclid(height)).rem_euclid(height);

        self
    }

    fn quadrant(&self, width: i32, height: i32) -> Option<Quadrant> {
        match self.position {
            (w, h) if w < width / 2 && h < height / 2 => Some(Quadrant::TopLeft),
            (w, h) if w < width / 2 && h > height / 2 => Some(Quadrant::TopRight),
            (w, h) if w > width / 2 && h < height / 2 => Some(Quadrant::BotLeft),
            (w, h) if w > width / 2 && h > height / 2 => Some(Quadrant::BotRight),
            _ => None,
        }
    }
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (start_x, start_y)) = preceded(
        tag("p="),
        separated_pair(complete::i32, tag(","), complete::i32),
    )(input)?;

    let (input, (vel_x, vel_y)) = preceded(
        tag(" v="),
        separated_pair(complete::i32, tag(","), complete::i32),
    )(input)?;

    Ok((
        input,
        Robot {
            position: (start_x, start_y),
            vel: (vel_x, vel_y),
        },
    ))
}

fn parse_robots(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, parse_robot)(input)
}

fn process(input: &str, width: i32, height: i32) -> miette::Result<String> {
    let (_input, mut robots) = parse_robots(input).map_err(|e| miette!("parser failed {}", e))?;

    let mut stdout = stdout();

    stdout.execute(cursor::Hide).unwrap();

    for i in 0..(width * height) {
        robots = robots
            .into_iter()
            .map(|robot| robot.simulate(width, height, 1))
            .collect::<Vec<_>>();

        if robots.iter().map(|robot| robot.position).count()
            != robots.iter().map(|robot| robot.position).unique().count()
        {
            continue;
        }

        let locs = robots
            .iter()
            .map(|robot| robot.position)
            .collect::<BTreeSet<_>>();

        let string = (0..width)
            .map(|col| {
                let locs = locs.clone();
                (0..height)
                    .map(
                        move |row| {
                            if locs.contains(&(row, col)) {
                                '0'
                            } else {
                                ' '
                            }
                        },
                    )
                    .collect::<String>()
            })
            .join("\n");

        stdout.queue(cursor::SavePosition).unwrap();
        stdout
            .write_all(format!("{}\n{}", string, i).as_bytes())
            .unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        // thread::sleep(time::Duration::from_millis(500));
        let _ = stdout.flush();
        let _ = stdin().read_line(&mut String::new());

        stdout.queue(cursor::RestorePosition).unwrap();
        stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
    }
    stdout.execute(cursor::Show).unwrap();

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input, 11, 7)?);
        Ok(())
    }
}
