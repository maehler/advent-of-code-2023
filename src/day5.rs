use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct AlmanacMap {
    from: String,
    to: String,
    map: Vec<(u64, u64, u64)>,
}

impl AlmanacMap {
    fn new(from: String, to: String, map: Vec<(u64, u64, u64)>) -> Self {
        Self { from, to, map }
    }

    fn map(&self, num: u64) -> u64 {
        for map in self.map.iter() {
            if num >= map.1 && num <= map.1 + map.2 {
                let diff = num - map.1;
                return map.0 + diff;
            }
        }

        num
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn new(seeds: Vec<u64>, maps: Vec<AlmanacMap>) -> Self {
        Self { seeds, maps }
    }

    fn walk(&self) -> Vec<u64> {
        let mut res = vec![];
        for seed in self.seeds.iter() {
            let mut current_number = *seed;
            for map in self.maps.iter() {
                current_number = map.map(current_number);
            }
            res.push(current_number);
        }
        res
    }
}

mod almanac_parser {
    use nom::{
        IResult,
        bytes::complete::{tag, take_until1},
        character::complete::{digit1, multispace0, space1, newline},
        multi::{many1, separated_list1}
    };

    use super::{Almanac, AlmanacMap};

    fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = separated_list1(space1, digit1)(input)?;
        let (input, _) = newline(input)?;

        Ok((input, seeds.iter().map(|s| s.parse::<u64>().unwrap()).collect()))
    }

    fn parse_map_header(input: &str) -> IResult<&str, (&str, &str)> {
        let (input, from_type) = take_until1("-")(input)?;
        let (input, _) = tag("-to-")(input)?;
        let (input, to_type) = take_until1(" ")(input)?;
        let (input, _) = tag(" map:")(input)?;
        let (input, _) = newline(input)?;

        Ok((input, (from_type, to_type)))
    }

    fn parse_map(input: &str) -> IResult<&str, (u64, u64, u64)> {
        let (input, dest_start) = digit1(input)?;
        let (input, _) = space1(input)?;
        let (input, source_start) = digit1(input)?;
        let (input, _) = space1(input)?;
        let (input, width) = digit1(input)?;
        let (input, _) = newline(input)?;

        Ok((
            input,
            (
                dest_start.parse::<u64>().unwrap(),
                source_start.parse::<u64>().unwrap(),
                width.parse::<u64>().unwrap()
            )
        ))
    }

    fn parse_map_block(input: &str) -> IResult<&str, (&str, &str, Vec<(u64, u64, u64)>)> {
        let (input, (from, to)) = parse_map_header(input)?;
        let (input, maps) = many1(parse_map)(input)?;
        let (input, _) = multispace0(input)?;

        Ok((input, (from, to, maps)))
    }

    pub fn parse(s: &str) -> Result<Almanac, nom::Err<nom::error::Error<&str>>> {
        let (input, seeds) = parse_seeds(s)?;
        let (input, _) = newline(input)?;
        let (_, maps) = many1(parse_map_block)(input)?;
        
        let almanac = Almanac::new(
            seeds,
            maps.iter()
                .map(|m| AlmanacMap::new(m.0.to_string(), m.1.to_string(), m.2.clone())).collect());

        Ok(almanac)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_seed_line() {
            let (input, seeds) = parse_seeds("seeds: 1 2 3\n").unwrap();    
            assert!(input.len() == 0);
            assert!(seeds == vec![1, 2, 3]);
        }

        #[test]
        fn test_map_header() {
            let (input, (from, to)) = parse_map_header("seed-to-soil map:\n").unwrap();
            assert!(input.len() == 0);
            assert!(from == "seed");
            assert!(to == "soil");

            let (input, (from, to)) = parse_map_header("mish-to-mash map:\n").unwrap();
            assert!(input.len() == 0);
            assert!(from == "mish");
            assert!(to == "mash");
        }

        #[test]
        fn test_map() {
            let (input, (dest, source, width)) = parse_map("1 2 3\n").unwrap();
            assert!(input.len() == 0);
            assert!(dest == 1);
            assert!(source == 2);
            assert!(width == 3);
        }
    }
}

fn read_almanac(input: &str) -> Almanac {
    let almanac_string = read_to_string(input).unwrap();
    let almanac = almanac_parser::parse(&almanac_string).unwrap();
    almanac.clone()
}

pub fn run(part: u8, input: String) {
    match part {
        1 => {
            let almanac = read_almanac(&input);
            let final_pos = almanac.walk();
            println!("{:?}", final_pos.iter().min().unwrap());
        },
        _ => println!("part {} not implemented for day 5", part),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almanac_map() {
        let map = AlmanacMap::new("seed".to_string(), "soil".to_string(), vec![(1, 10, 2)]);
        assert_eq!(map.map(5), 5);
        assert_eq!(map.map(11), 2);

        let map = AlmanacMap::new("seed".to_string(), "soil".to_string(), vec![(50, 98, 2), (52, 50, 48)]);
        assert_eq!(map.map(49), 49);
        assert_eq!(map.map(50), 52);
        assert_eq!(map.map(97), 99);
        assert_eq!(map.map(98), 50);
    }
}
