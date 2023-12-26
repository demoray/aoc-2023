use std::io::{stdin, BufRead, BufReader, Read};

fn main() {
    let result = run(stdin().lock());
    println!("{result:?}");
}

fn parse_line(line: String) -> Option<usize> {
    line.split(':').nth(1)?.replace(' ', "").parse().ok()
}

fn run<R>(handle: R) -> Option<usize>
where
    R: Read,
{
    let mut lines: Vec<String> = BufReader::new(handle)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let times = parse_line(lines.remove(0));
    let records = parse_line(lines.remove(0));

    println!("times: {:?}", times);
    println!("records: {:?}", records);

    let mut win_totals = vec![];
    for (time, record) in times.iter().zip(records.iter()) {
        let mut wins = vec![];
        for speed in 1..*time {
            let move_time = time - speed;
            let distance = move_time * speed;
            if distance > *record {
                wins.push(speed);
            }
        }
        win_totals.push(wins.len());
        //println!("wins: {wins:?} - {}", wins.len());
    }

    let result = win_totals.iter().fold(1, |acc, x| acc * x);
    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_06() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?);
        assert_eq!(result, Some(71503));
        Ok(())
    }
}
