use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader, Read},
};

const DICE: &[(&str, usize)] = &[("red", 12), ("green", 13), ("blue", 14)];

fn main() -> Result<()> {
    let result = run(stdin().lock())?;
    println!("{result}");
    Ok(())
}

fn run<R>(handle: R) -> Result<usize>
where
    R: Read,
{
    Ok(BufReader::new(handle)
        .lines()
        .filter_map(|l| l.ok())
        .map(|x| parse_game(&x))
        .filter_map(|x| x.ok().flatten())
        .sum())
}

fn parse_game(line: &str) -> Result<Option<usize>> {
    let game = line
        .split(':')
        .nth(0)
        .ok_or_else(|| anyhow!("anyhow before ':'"))?
        .split(' ')
        .nth(1)
        .ok_or_else(|| anyhow!("unable to find game"))?
        .trim()
        .parse()?;

    let sets = line
        .split(':')
        .nth(1)
        .ok_or_else(|| anyhow!("uable to find matches"))?
        .trim()
        .split(';')
        .map(|x| x.trim())
        .filter_map(parse_matches)
        .collect::<Vec<_>>();

    for set in sets {
        if !check_set(set) {
            return Ok(None);
        }
    }

    Ok(Some(game))
}

fn check_set(totals: HashMap<&str, usize>) -> bool {
    for entry in totals {
        if let Some((_, value)) = DICE.iter().find(|x| x.0 == entry.0) {
            if entry.1 > *value {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn parse_matches(dice: &str) -> Option<HashMap<&str, usize>> {
    let mut sets = HashMap::new();
    for mut entry in dice.split(',').map(|x| x.trim()).map(|x| x.split(' ')) {
        let count: usize = entry.nth(0).unwrap().parse().unwrap();
        let name = entry.last().unwrap();
        *sets.entry(name).or_insert(0) += count;
    }
    Some(sets)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_02() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?)?;
        assert_eq!(result, 8);
        Ok(())
    }
}
