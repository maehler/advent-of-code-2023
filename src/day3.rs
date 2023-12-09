use std::collections::HashMap;
use std::cmp;

use super::utils;

#[derive(Debug, Eq, PartialEq)]
enum Content {
    Part(u32),
    Symbol,
    Gear
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Schematic {
    positions: HashMap<Pos, Content>
}

impl Schematic {
    fn new() -> Self {
        Self { positions: HashMap::new() }
    }

    fn find_parts(self) -> Vec<u32> {
        let mut parts = vec![];
        for (pos, x) in self.positions.iter() {
            match x {
                Content::Part(x) => {
                    //let num_length = x.ilog10() as usize + 1;
                    //let min_x = cmp::max(0, (pos.x as i32) - num_length as i32) as usize;
                    let min_x = cmp::max(0, (pos.x as i32) - 1) as usize;
                    let min_y = cmp::max(0, (pos.y as i32) - 1) as usize;

                    for i in min_x..=(pos.x + 1) {
                        for j in min_y..=(pos.y + 1) {
                            if let Some(neighbor) = self.positions.get(&Pos::new(i, j)) {
                                if (neighbor == &Content::Symbol || neighbor == &Content::Gear) && !parts.contains(x) {
                                    parts.push(*x);
                                }
                            }
                        }
                    }
                },
                Content::Gear => continue,
                Content::Symbol => continue
            }
        }
        parts
    }

    fn find_gear_ratios(self) -> Vec<(u32, u32)> {
        let mut gear_ratios = vec![];
        for (pos, x) in self.positions.iter() {
            if x == &Content::Gear {
                let min_x = cmp::max(0, (pos.x as i32) - 1) as usize;
                let min_y = cmp::max(0, (pos.y as i32) - 1) as usize;
                let mut neighboring_parts = vec![];
                for i in min_x..=(pos.x + 1) {
                    for j in min_y..=(pos.y + 1) {
                        match self.positions.get(&Pos::new(i, j)) {
                            Some(Content::Part(n)) => {
                                if !neighboring_parts.contains(n) {
                                    neighboring_parts.push(*n);
                                }
                            }
                            _ => continue
                        }
                    }
                }
                if neighboring_parts.len() == 2 {
                    gear_ratios.push((neighboring_parts[0], neighboring_parts[1]));
                }
            }
        }
        gear_ratios
    }
}

fn parse_schematic(input: &str) -> std::io::Result<(Schematic, HashMap<u32, u32>)> {
    let lines = utils::read_lines(input).unwrap();
    let mut number_id = 0;
    let mut numbers = HashMap::new();
    let mut schematic = Schematic::new();

    for (y, line) in lines.enumerate() {
        let linelen = &line.as_ref().unwrap().len();
        let mut number: Vec<char> = vec![];
        line?.chars().enumerate().for_each(|(x, c)| {
            if c.is_digit(10) {
                if number.is_empty() {
                    number_id += 1;
                }
                number.push(c);
                let pos = Pos::new(x, y);
                schematic.positions.insert(
                    pos,
                    Content::Part(number_id)
                );
            } else {
                if !number.is_empty() {
                    numbers.insert(number_id, number.iter().collect::<String>().parse::<u32>().unwrap());
                    number.clear();
                }

                if c != '.' && c != '*' {
                    let pos = Pos::new(x, y);
                    schematic.positions.insert(pos, Content::Symbol);
                } else if c == '*' {
                    let pos = Pos::new(x, y);
                    schematic.positions.insert(pos, Content::Gear);
                }
            }
        });
        if !number.is_empty() {
            let pos = Pos::new(linelen - 1, y);
            schematic.positions.insert(
                pos,
                Content::Part(number.iter().collect::<String>().parse::<u32>().unwrap())
            );
            numbers.insert(number_id, number.iter().collect::<String>().parse::<u32>().unwrap());
        } 
    }

    Ok((schematic, numbers))
}

pub fn run(part: u8, input: String) {
    match part {
        1 => {
            let (schematic, numbers) = parse_schematic(&input).unwrap();
            let parts = schematic.find_parts();
            let res = parts.iter().map(|x| numbers.get(&x).unwrap()).sum::<u32>();
            println!("{:?}", res);
        }
        2 => {
            let (schematic, numbers) = parse_schematic(&input).unwrap();
            let gear_ratios = schematic.find_gear_ratios();
            println!(
                "{:?}",
                gear_ratios
                    .iter()
                    .map(|(a, b)| numbers.get(a).unwrap() * numbers.get(b).unwrap())
                    .sum::<u32>()
            );
        }
        _ => {
            println!("Part {} not implemented for day 3", part);
        }
    }
}
