use std::collections::HashMap;
use std::cmp;

use super::utils;

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
    positions: HashMap<Pos, u32>
}

impl Schematic {
    fn new() -> Self {
        Self { positions: HashMap::new() }
    }

    fn find_parts(self) -> Vec<u32> {
        let mut parts = vec![];
        for (pos, x) in self.positions.iter() {
            if x == &0 {
                continue;
            }
            let num_length = x.ilog10() as usize + 1;
            let min_x = cmp::max(0, (pos.x as i32) - num_length as i32) as usize;
            let min_y = cmp::max(0, (pos.y as i32) - 1) as usize;
            for i in min_x..=(pos.x + 1) {
                for j in min_y..=(pos.y + 1) {
                    if let Some(neighbor) = self.positions.get(&Pos::new(i, j)) {
                        if neighbor == &0 {
                            parts.push(*x);
                        }
                    }
                }
            }
        }
        parts
    }
}

fn parse_schematic(input: &str) -> std::io::Result<Schematic> {
    let lines = utils::read_lines(input).unwrap();
    let mut schematic = Schematic::new();

    for (y, line) in lines.enumerate() {
        let linelen = &line.as_ref().unwrap().len();
        let mut number: Vec<char> = vec![];
        line?.chars().enumerate().for_each(|(x, c)| {
            if c.is_digit(10) {
                number.push(c);
            } else {
                if number.len() > 0 {
                    let pos = Pos::new(x - 1, y);
                    schematic.positions.insert(pos, number.iter().collect::<String>().parse::<u32>().unwrap());
                    number.clear();
                }

                if c != '.' {
                    let pos = Pos::new(x, y);
                    schematic.positions.insert(pos, 0);
                }
            }
        });
        if number.len() > 0 {
            let pos = Pos::new(linelen - 1, y);
            schematic.positions.insert(pos, number.iter().collect::<String>().parse::<u32>().unwrap());
        } 
    }

    Ok(schematic)
}

pub fn run(part: u8, input: String) {
    match part {
        1 => {
            let schematic = parse_schematic(&input);
            let parts = schematic.unwrap().find_parts();
            println!("{}", parts.iter().sum::<u32>());
        }
        _ => {
            println!("Part {} not implemented for day 3", part);
        }
    }
}
