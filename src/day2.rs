use std::collections::HashMap;

use super::utils;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Color {
    Blue,
    Red,
    Green
}

type Draw = (Color, u32);

fn parse_colors(s: &str) -> Option<Draw> {
    let components = s.trim()
        .split(" ")
        .collect::<Vec<&str>>();

    match components.as_slice() {
        [n, "red"] => Some((Color::Red, n.parse::<u32>().unwrap())),
        [n, "blue"] => Some((Color::Blue, n.parse::<u32>().unwrap())),
        [n, "green"] => Some((Color::Green, n.parse::<u32>().unwrap())),
        _ => None
    }
}

fn parse_input(input: &str) -> Result<Vec<Vec<Vec<Draw>>>, std::io::Error> {
    let lines = utils::read_lines(input)?;
    let mut games = vec![];
    for line in lines {
        if let Ok(line) = line {
            let res = line.trim_start_matches("Game ")
                .split(":")
                .collect::<Vec<&str>>();

            let _ = res[0].parse::<u32>().unwrap();

            let x = res[1].trim()
                .split(";")
                .map(|s| {
                    s.split(", ").map(|d| parse_colors(d).unwrap()).collect::<Vec<_>>()
                }).collect::<Vec<_>>();

            games.push(x);
        }
    }
    Ok(games)
}

fn check_games(games: Vec<Vec<Vec<Draw>>>, totals: HashMap<Color, u32>) -> u32 {
    let mut score: u32 = 0;
    for (game_id, game) in games.iter().enumerate() {
        let mut game_ok = true;
        for round in game {
            let mut counts_below = round.iter().map(|(c, n)| {
                totals.get(&c).unwrap() >= n
            });
            if !counts_below.all(|x| x) {
                game_ok = false;
            }
        }

        if game_ok {
            score += game_id as u32 + 1;
        }
    }
    score
}

fn calculate_minimum(games: Vec<Vec<Vec<Draw>>>) -> u32 {
    let mut sum = 0;
    for game in games {
        let mut max = HashMap::from([
            (Color::Red, 0),
            (Color::Green, 0),
            (Color::Blue, 0)
        ]);
        for round in game {
            for draw in round {
                if max.get(&draw.0).unwrap() < &draw.1 {
                    max.insert(draw.0, draw.1);
                }
            }
        }
        let power = max.values().fold(1, |acc, x| acc * x);
        sum += power;
    }
    sum
}

pub fn run(part: u8, input: String) {
    let totals = HashMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14),
    ]);

    match part {
        1 => {
            let games = parse_input(&input).unwrap();
            let res = check_games(games, totals);
            println!("{}", res);
        }
        2 => {
            let games = parse_input(&input).unwrap();
            let res = calculate_minimum(games);
            println!("{}", res);
        }
        _ => {
            println!("Part {} not implemented for day 2", part);
        }
    }
}
