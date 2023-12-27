use std::{
    collections::BTreeMap,
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
    let mut lines: Vec<String> = BufReader::new(handle)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let inst = lines.remove(0);

    let mut maps = BTreeMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (key, rest) = line.split_once(" = ")?;
        let key = key.trim().to_owned();
        let rest = rest.trim().to_owned();
        let (a, b) = rest[1..rest.len() - 1].split_once(", ")?;
        maps.insert(key, (a.to_owned(), b.to_owned()));
    }

    println!("inst: {inst}");
    println!("maps: {maps:?}");

    let mut steps = 0;
    let mut current = "AAA";
    for i in std::iter::repeat(inst.chars().into_iter()) {
        for x in i {
            steps += 1;
            println!("current: {current}, x: {x} steps: {steps}");
            let (a, b) = maps.get(current)?;
            current = match x {
                'L' => a,
                'R' => b,
                _ => {
                    unimplemented!("invalid step {x}");
                }
            };

            if current == "ZZZ" {
                return Some(steps);
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_08() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?);
        assert_eq!(result, Some(2));

        let result = run(File::open("data/second.txt")?);
        assert_eq!(result, Some(6));
        Ok(())
    }
}
