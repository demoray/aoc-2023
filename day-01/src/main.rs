use std::io::{stdin, BufRead, BufReader, Read};

fn main() {
    let result = run(stdin().lock());
    println!("{result}");
}

fn run<R>(handle: R) -> u32
where
    R: Read,
{
    BufReader::new(handle)
        .lines()
        .filter_map(|l| value(l.ok()?))
        .sum()
}

fn value(line: String) -> Option<u32> {
    let mut iter = line.chars().filter_map(|c| c.to_digit(10));
    let first = iter.nth(0)?;
    let last = iter.last().unwrap_or(first);
    Some(first * 10 + last)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_01() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?);
        assert_eq!(result, 142);
        Ok(())
    }
}
