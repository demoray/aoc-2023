use anyhow::Result;
use std::io::{stdin, BufRead, BufReader, Read};

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
    let mut result = Vec::new();
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

    for (line_number, offset, value) in potential {
        let mut is_part = false;
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
                if c.is_digit(10) || c == '.' {
                    continue;
                }
                is_part = true;
            }
            println!(
                "checking offsets {line} {check_lines:?} {check_offsets:?} - {value} {is_part:?}"
            );
        }
        if is_part {
            result.push(value.parse()?);
        }
    }
    println!("results {result:?}");

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_03() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?)?;
        assert_eq!(result, 4361);
        Ok(())
    }
}
