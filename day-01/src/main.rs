use std::io::{stdin, BufRead, BufReader};

fn main() {
    let result: u32 = BufReader::new(stdin().lock())
        .lines()
        .filter_map(|l| value(l.ok()?))
        .sum();
    println!("{result}");
}

fn value(line: String) -> Option<u32> {
    let mut iter = line.chars().filter_map(|c| c.to_digit(10));
    let first = iter.nth(0)?;
    let last = iter.last().unwrap_or(first);
    Some(first * 10 + last)
}
