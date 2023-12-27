use std::{
    collections::BTreeSet,
    io::{stdin, BufRead, BufReader, Read},
};

fn main() {
    let result = run(stdin().lock());
    println!("{result}");
}

fn run<R>(handle: R) -> usize
where
    R: Read,
{
    BufReader::new(handle)
        .lines()
        .map_while(Result::ok)
        .filter_map(card)
        .sum()
}

fn card(line: String) -> Option<usize> {
    let game = line.split(':').nth(1)?;

    let winners = game
        .split('|')
        .next()?
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
    if wins.is_empty() {
        println!("line: {line} winners {winners:?} picks {picks:?} no wins");
        None
    } else {
        Some(1 << (wins.len() - 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_04() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?);
        assert_eq!(result, 13);
        Ok(())
    }
}
