use aoc2021::read_lines;

fn main() -> std::io::Result<()> {
    let day1_1 = read_lines("day1.input")
        .expect("input file day1.input missing")
        .map(|line| line.unwrap().parse::<u32>().ok())
        .fold(None, |acc, next| {
            acc.map(|(sum, prev)| { if prev < next { (sum + 1, next) } else { (sum, next) } }).or(Some((0, next)))
        })
        .map(|(x, _)| x)
        .unwrap_or(0); // we got an empty list, so we can return 0

    println!("day1_1 = {}", day1_1);

    let day1_2 = read_lines("day1.input")
        .expect("input file day1.input missing")
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|inner| inner.into_iter().sum())
        .fold(None, |acc: Option<(u32, u32)>, next| {
            acc.map(|(sum, prev)| { if prev < next { (sum + 1, next) } else { (sum, next) } }).or(Some((0, next)))
        })
        .map(|(x, _)| x)
        .unwrap_or(0); // we got an empty list, so we can return 0

    println!("day1_2 = {}", day1_2);

    Ok(())
}
