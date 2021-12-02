use aoc2021::{read_lines, depth_calculator::*};

fn main() -> std::io::Result<()> {

    let day2_1 = read_lines("day2.input")
        .expect("input file day2.input missing")
        .flat_map(|line| line.map(|val| Movement::try_from(val).ok()))
        .fold(Position { depth: 0, horizontal: 0 , aim: 0}, |acc, movement| {
            match movement {
                Some(Movement { direction: Direction::Up, step }) => Position {
                    depth: acc.depth - step,
                    horizontal: acc.horizontal,
                    aim: 0,
                },
                Some(Movement { direction: Direction::Down, step }) => Position {
                    depth: acc.depth + step,
                    horizontal: acc.horizontal,
                    aim: 0,
                },
                Some(Movement { direction: Direction::Forward, step }) => Position {
                    depth: acc.depth,
                    horizontal: acc.horizontal + step,
                    aim: 0,
                },
                _ => acc,
            }
        });

    println!("day2_1 = {}", (day2_1.horizontal * day2_1.depth));

    let day2_2 = read_lines("day2.input")
        .expect("input file day2.input missing")
        .flat_map(|line| line.map(|val| Movement::try_from(val).ok()))
        .fold(Position { depth: 0, horizontal: 0 , aim: 0}, |acc, movement| {
            match movement {
                Some(Movement { direction: Direction::Up, step }) => Position {
                    depth: acc.depth,
                    horizontal: acc.horizontal,
                    aim: acc.aim - step,
                },
                Some(Movement { direction: Direction::Down, step }) => Position {
                    depth: acc.depth,
                    horizontal: acc.horizontal,
                    aim: acc.aim + step,
                },
                Some(Movement { direction: Direction::Forward, step }) => Position {
                    depth: acc.depth + (step * acc.aim),
                    horizontal: acc.horizontal + step,
                    aim: acc.aim,
                },
                _ => acc,
            }
        });

    println!("day2_2 = {}", (day2_2.horizontal * day2_2.depth));

    Ok(())
}

