use anyhow::Result;
use std::{
    collections::HashSet,
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
    let lines: Vec<_> = BufReader::new(handle).lines().collect();
    let lines = lines.into_iter().collect::<Result<Vec<_>, _>>()?;
    let parts = get_parts(&lines)?;
    Ok(parts.into_iter().sum())
}

fn get_parts(lines: &[String]) -> Result<Vec<usize>> {
    let mut result = HashSet::new();
    let mut potential = Vec::new();

    for (line_number, line) in lines.iter().enumerate() {
        let mut current: Option<(usize, String)> = None;
        for (index, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if let Some(current) = &mut current {
                    current.1.push(c);
                } else {
                    current = Some((index, c.to_string()));
                }
            } else if let Some((offset, value)) = current.take() {
                potential.push((line_number, offset, value));
            }
        }
        if let Some((offset, value)) = current.take() {
            potential.push((line_number, offset, value));
        }
    }

    // println!("potential {potential:?}");

    let mut gears = Vec::new();

    for (line_number, offset, value) in potential {
        let mut part_gears = Vec::new();
        let top_left = (line_number.saturating_sub(1), offset.saturating_sub(1));
        let bottom_right = (line_number + 1, offset + value.len() + 1);
        let check_lines = top_left.0..=bottom_right.0;
        let check_offsets = top_left.1..bottom_right.1;
        for check_line in check_lines.clone() {
            let Some(line) = lines.get(check_line) else {
                continue;
            };
            for check_offset in check_offsets.clone() {
                let Some(c) = line.chars().nth(check_offset) else {
                    continue;
                };
                if c == '*' {
                    part_gears.push((check_line, check_offset));
                }
            }
        }
        // println!("checking offsets {check_lines:?} {check_offsets:?} - {value} {is_part:?}");
        if !part_gears.is_empty() {
            let value: usize = value.parse()?;
            gears.push((value, part_gears));
        }
    }

    for (a_value, a_parts) in &gears {
        for (b_value, b_parts) in &gears {
            if a_value == b_value {
                continue;
            }
            for a_part in a_parts {
                if b_parts.contains(a_part) {
                    if result.contains(&(b_value, a_value)) {
                        continue;
                    }
                    result.insert((a_value, b_value));
                }
            }
        }
    }

    println!("gears {gears:?}");
    println!("results {result:?}");

    Ok(result.into_iter().map(|(a, b)| a * b).collect())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_03_02() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?)?;
        assert_eq!(result, 467835);
        Ok(())
    }
}
