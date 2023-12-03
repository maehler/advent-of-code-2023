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

pub fn run(input: String) {
    let ints = parse(&input);
    
    if let Ok(ints) = ints {
        println!("{}", ints.iter().sum::<u32>());
    } else {
        println!("Could not parse input");
    }
}
