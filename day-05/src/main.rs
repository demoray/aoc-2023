use std::{
    collections::BTreeMap,
    io::{stdin, BufRead, BufReader, Read},
    ops::Range,
};

type Maps = BTreeMap<Option<String>, Vec<(Range<usize>, Range<usize>)>>;

const KEYS: &[&str] = &[
    "seed-to-soil map:",
    "soil-to-fertilizer map:",
    "fertilizer-to-water map:",
    "water-to-light map:",
    "light-to-temperature map:",
    "temperature-to-humidity map:",
    "humidity-to-location map:",
];

fn main() {
    let result = run(stdin().lock());
    println!("{result:?}");
}

fn run<R>(handle: R) -> Option<usize>
where
    R: Read,
{
    let mut lines: Vec<String> = BufReader::new(handle)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let seeds = lines
        .remove(0)
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("seeds {seeds:?}");

    let mut current = None;
    let mut ranges: Maps = BTreeMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.chars().next()?.is_alphabetic() {
            // println!("set current");
            current = Some(line);
        } else {
            let values = line
                .split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            // println!("values: {values:?}");
            let dst = values[0];
            let src = values[1];
            let size = values[2];

            ranges
                .entry(current.clone())
                .or_default()
                .push((src..src + size, dst..dst + size));
        }
    }

    let mut lowest = None;
    // println!("ranges: {ranges:#?}",);
    for seed in seeds {
        let x = map_it(seed, &ranges);
        if lowest.is_none() || x < lowest.unwrap() {
            lowest = Some(x);
        }
    }

    lowest
}

fn map_it(seed: usize, maps: &Maps) -> usize {
    let mut current = seed;
    println!("seed {seed}");
    for entry in KEYS {
        println!("entry: {entry:?}");
        let ranges = maps.get(&Some(entry.to_string())).unwrap();
        for (src, dst) in ranges {
            if src.contains(&current) {
                let offset = current - src.start;
                current = dst.start + offset;
                // println!(" offset:{offset} {src:?} {dst:?}");
                break;
            }
        }
        println!(" current {current}");
    }

    current
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_05() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?);
        assert_eq!(result, Some(35));
        Ok(())
    }
}
