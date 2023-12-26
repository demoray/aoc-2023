use std::{
    collections::{BTreeMap, BTreeSet},
    io::{stdin, BufRead, BufReader, Read},
};

fn main() {
    let result = run(stdin().lock());
    println!("{result:?}");
}

fn run<R>(handle: R) -> Option<usize>
where
    R: Read,
{
    let lines = BufReader::new(handle)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let mut count = 0;

    let mut copies = Vec::new();
    for i in 0..lines.len() {
        copies.push((i, 1));
    }
    println!("copies {copies:?}");

    while !copies.is_empty() {
        let (i, card_count) = copies.remove(0);
        let line = lines.get(i)?;
        if let Some(c) = card(line) {
            for i in 0..c {
                if let Some(e) = copies.get_mut(i) {
                    e.1 += card_count;
                }
            }
        }
        count += card_count;
    }

    Some(count)
}

fn card(line: &str) -> Option<usize> {
    let game = line.split(':').nth(1)?;

    let winners = game
        .split('|')
        .nth(0)?
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<BTreeSet<_>>();

    let picks = game
        .split('|')
        .nth(1)?
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<BTreeSet<_>>();

    let wins = picks.intersection(&winners).collect::<Vec<_>>();
    Some(wins.len())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_04_02() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?);
        assert_eq!(result, Some(30));
        Ok(())
    }
}
