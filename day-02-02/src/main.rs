use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader, Read},
};

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
        .filter_map(|x| x.ok())
        .sum())
}

fn parse_game(line: &str) -> Result<usize> {
    let sets = line
        .split(':')
        .nth(1)
        .ok_or_else(|| anyhow!("uable to find matches"))?
        .trim()
        .split(';')
        .map(|x| x.trim())
        .filter_map(parse_matches)
        .collect::<Vec<_>>();

    let mut mins = HashMap::new();

    for set in sets {
        for (name, value) in set {
            let entry = mins.entry(name).or_insert(0_usize);
            if value > *entry {
                *entry = value;
            }
        }
    }
    let result = mins.values().fold(1, |acc, v| acc * v);

    Ok(result)
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
    fn test_02_02() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?)?;
        assert_eq!(result, 2286);
        Ok(())
    }
}
