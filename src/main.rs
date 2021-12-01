use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let day1_1 = read_lines("day1_1.input")
        .expect("input file day1_1.input missing")
        .map(|line| line.unwrap().parse::<u32>().ok())
        .fold(None, |acc, next| {
            acc.map(|(sum, prev)| { if prev < next { (sum + 1, next) } else { (sum, next) } }).or(Some((0, next)))
        })
        .map(|(x, _)| x)
        .unwrap_or(0); // we got an empty list, so we can return 0

    println!("{}", day1_1);

    let day1_2 = read_lines("day1_2.input")
        .expect("input file day1_2.input missing")
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|inner| inner.into_iter().sum())
        .fold(None, |acc: Option<(u32, u32)>, next| {
            acc.map(|(sum, prev)| { if prev < next { (sum + 1, next) } else { (sum, next) } }).or(Some((0, next)))
        })
        .map(|(x, _)| x)
        .unwrap_or(0); // we got an empty list, so we can return 0

    println!("{}", day1_2);

    Ok(())
}
