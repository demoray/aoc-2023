use std::{
    collections::BTreeMap,
    io::{stdin, BufRead, BufReader, Read},
    ops::Range,
};

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

    let seeds_base = lines
        .remove(0)
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    // println!("seeds {seeds:?}");

    let mut current = None;
    let mut ranges: BTreeMap<String, Vec<(Range<usize>, Range<usize>)>> = BTreeMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.chars().nth(0)?.is_alphabetic() {
            // println!("set current");
            current = Some(line);
        } else {
            let mut values = line
                .split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            // println!("values: {values:?}");
            let dst = values[0];
            let src = values[1];
            let size = values[2];

            ranges
                .entry(current.clone().unwrap())
                .or_default()
                .push((src..src + size, dst..dst + size));
        }
    }

    let mut lowest = None;
    for entry in seeds_base.chunks_exact(2) {
        let x = entry[0]..entry[0] + entry[1];
        for seed in x {
            let x = map_it(seed, &ranges);
            if lowest.is_none() || x < lowest.unwrap() {
                lowest = Some(x);
                println!("lowest {lowest:?}");
            }
        }
    }

    lowest
}

fn map_it(seed: usize, maps: &BTreeMap<String, Vec<(Range<usize>, Range<usize>)>>) -> usize {
    let mut current = seed;
    // println!("seed {seed}");
    for entry in KEYS {
        // println!("entry: {entry:?}");
        let ranges = maps.get(*entry).unwrap();
        for (src, dst) in ranges {
            if src.contains(&current) {
                let offset = current - src.start;
                current = dst.start + offset;
                // println!(" offset:{offset} {src:?} {dst:?}");
                break;
            }
        }
        // println!(" current {current}");
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
        assert_eq!(result, Some(46));
        Ok(())
    }
}