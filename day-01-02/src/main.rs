use std::io::{stdin, BufRead, BufReader, Read};

const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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
        .map_while(Result::ok)
        .filter_map(value)
        .inspect(|v| println!("{:?}", v))
        .sum()
}

fn value(line: String) -> Option<u32> {
    let mut iter = line
        .chars()
        .enumerate()
        .filter_map(|c| Some((c.0, c.1.to_digit(10)?)));

    let mut first_digit = iter.next();
    let mut last_digit = iter.last().or(first_digit);

    for (i, value) in DIGITS.iter().enumerate().map(|(i, v)| (i + 1, *v)) {
        let iter = line.match_indices(value);
        for (offset, _) in iter.clone() {
            if first_digit.is_none() || offset < first_digit.unwrap().0 {
                first_digit = Some((offset, i as u32));
            }
            if last_digit.is_none() || offset > last_digit.unwrap().0 {
                last_digit = Some((offset, i as u32));
            }
        }
    }
    let first = first_digit?.1;
    let last = last_digit.unwrap_or(first_digit?).1;
    Some(first * 10 + last)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_01_02() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?);
        println!("result: {result}");
        assert_eq!(result, 281);
        Ok(())
    }
}
