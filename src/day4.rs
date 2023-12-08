use std::collections::HashSet;

use super::utils;

mod card_parser {
    use nom::{
        character::complete::{
            space0,
            space1,
        },
        multi::separated_list1,
        bytes::complete::{tag, take_while1},
        IResult
    };

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    pub fn parse_card(input: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = space0(input)?;
        let (input, n) = take_while1(is_digit)(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space0(input)?;
        let (input, winners) = separated_list1(space1, take_while1(is_digit))(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("|")(input)?;
        let (input, _) = space0(input)?;
        let (input, numbers) = separated_list1(space1, take_while1(is_digit))(input)?;

        let n = n.parse::<u32>().unwrap();
        let winners = winners.iter().map(|w| w.parse::<u32>().unwrap()).collect();
        let numbers = numbers.iter().map(|n| n.parse::<u32>().unwrap()).collect();

        Ok((input, (n, winners, numbers)))
    }

    pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
        let (_, (_, winners, numbers)) = parse_card(input).unwrap();
        (winners, numbers)
    }
}

fn parse_cards(input: &str) -> Vec<u32> {
    let lines = utils::read_lines(input).unwrap();

    let mut scores = vec![];

    for line in lines {
        let (winners, numbers) = card_parser::parse(&line.unwrap());
        let w: HashSet<u32> = HashSet::from_iter(winners);
        let n: HashSet<u32> = HashSet::from_iter(numbers);
        let n_winning = w.intersection(&n).count();
        if n_winning == 0 {
            scores.push(0);
        } else {
            scores.push(2_u32.pow(n_winning as u32 - 1));
        }
    }

    scores
}

pub fn run(part: u8, input: String) {
    match part {
        1 => {
            let scores = parse_cards(&input);    
            println!("{:?}", scores.iter().sum::<u32>());
        },
        _ => println!("part {} not implemented for day 4", part),
    }
}
