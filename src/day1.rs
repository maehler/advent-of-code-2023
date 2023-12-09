use std::fs;

fn parse(filename: &str) -> Result<Vec<u32>, std::io::Error> {
    let data = fs::read_to_string(filename)?;
    let mut ints = vec![];

    data.split("\n").for_each(|line| {
        let mut int = vec![];
        line.chars().for_each(|c| {
            if c.is_digit(10) {
                int.push(c);
            }
        });
        if int.is_empty() {
            return;
        }
        ints.push(format!("{}{}", int[0], int[int.len() - 1]).parse::<u32>().unwrap());
    });

    Ok(ints)
}

fn parse_line2(line: &str) -> Result<u32, std::io::Error> {
    let string_ints = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut ints = vec![];
    for (i, string_int) in string_ints.iter().enumerate() {
        if let Some(x) = line.find(string_int) {
            ints.push((x, i));
        }
        if let Some(y) = line.rfind(string_int) {
            if y != ints.last().unwrap().0 {
                ints.push((y, i));
            }
        }
    }
    line.chars().enumerate().filter(|c| c.1.is_digit(10)).for_each(|c| {
        ints.push((c.0, c.1.to_digit(10).unwrap() as usize));
    });

    ints.sort_by_key(|x| x.0);
    
    Ok(ints[0].1 as u32 * 10 + ints[ints.len() - 1].1 as u32)
}

fn parse2(filename: &str) -> Result<Vec<u32>, std::io::Error> {
    let data = fs::read_to_string(filename)?;
    let mut first_last_ints = vec![];

    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }
        first_last_ints.push(parse_line2(line)?);
    }

    Ok(first_last_ints)
}

pub fn run(part: u8, input: String) {
    match part {
        1 => {
            if let Ok(ints) = parse(&input) {
                println!("{}", ints.iter().sum::<u32>());
            } else {
                println!("Could not parse input");
            }
        }
        2 => {
            if let Ok(ints) = parse2(&input) {
                println!("{}", ints.iter().sum::<u32>());
            } else {
                println!("Could not parse input");
            }
        },
        _ => println!("Part {} not implemented for day 1", part),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_digits() {
        let res = parse_line2("oneone").unwrap();
        assert_eq!(res, 11);

        let res = parse_line2("eightwone").unwrap();
        assert_eq!(res, 81);
    }

    #[test]
    fn test_same_double_digits() {
        let res = parse_line2("onethreeone").unwrap();
        assert_eq!(res, 11);
    }
}
