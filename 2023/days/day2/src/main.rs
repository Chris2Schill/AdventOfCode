use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::cmp;

fn main() {
    let mut sum_game_ids = 0;
    let mut sum_min_bag_powers = 0;
    let bag: Bag = Bag {red:12, green:13, blue:14};
    if let Ok(lines) = read_lines("days/day2/day2.in") {
        for line in lines {
            let aline = &line.unwrap();

            let game = Game::from_str(aline);
            match game {
                Ok(g) => {
                    // part one
                    sum_game_ids += is_game_possible(&bag, &g);

                    // part two
                    let bag = min_bag_required_for_game(&g);
                    sum_min_bag_powers += power_of_bag(&bag);
                },
                Err(_) => {},
            }
        }
    }

    println!("Sum game ids:{sum_game_ids}");
    println!("Sum min bag powers:{sum_min_bag_powers}");
}

struct Bag
{
    red:   i32,
    green: i32,
    blue:  i32,
}

struct Game
{
    id: i32,
    bags: Vec<Bag>,
}

// Constructs a Bag from a string like "blue 1, green 3, red 4"
impl FromStr for Bag {
    type Err = std::num::ParseIntError;
    fn from_str(bag_str: &str) -> Result<Self, Self::Err> {
        let mut bag = Bag{red:0,green:0,blue:0};

        for pull in bag_str.split(',') {
            let tokens: Vec<_> = pull.split_whitespace().collect();
            if tokens.len() == 2 {
                let amount = tokens[0].parse::<i32>();
                let color  = tokens[1];
                match amount {
                    Ok(n) => {
                        match color {
                            "red"   => { bag.red   = n; },
                            "green" => { bag.green = n; },
                            "blue"  => { bag.blue  = n; },
                            _ => {},
                        }
                    }
                    Err(_) => {},
                }
            }
        }

        return Ok(bag);
    }
}

// Constructs a Game from a string like "Game 1: blue 1, green 3, red 4; blue 3, green 3"
impl FromStr for Game {
    type Err = std::string::ParseError;
    fn from_str(bag_str: &str) -> Result<Self, Self::Err> {
        let mut game: Game = Game{id: 0, bags:Vec::new() };

        let tokens: Vec<_> = bag_str.split(':').collect();
        if tokens.len() == 2 {
            let game_description: Vec<_> = tokens[0].split_whitespace().collect();
            let sub_games: Vec<_> = tokens[1].split(';').collect();


            let game_id = game_description.get(1);
            match game_id {
                Some(id) => {
                    game.id = id.parse::<i32>().unwrap();

                    for sub_game in sub_games {
                        let bag = Bag::from_str(sub_game);
                        match bag {
                            Ok(b) => game.bags.push(b),
                            Err(_) => {},
                        }
                    }
                },
                None => { /* return err */},
            }
        }
        return Ok(game);
    }
}


fn is_game_possible(bag: &Bag, game: &Game) -> i32 {
    for sub_bag in &game.bags {
        if sub_bag.red > bag.red || sub_bag.green > bag.green || sub_bag.blue > bag.blue {
            return 0;
        }
    }
    return game.id;
}

fn min_bag_required_for_game(game: &Game) -> Bag {
    let mut bag: Bag = Bag { red:0, green:0, blue:0 };

    for b in &game.bags {
        bag.red = cmp::max(b.red, bag.red);
        bag.green = cmp::max(b.green, bag.green);
        bag.blue = cmp::max(b.blue, bag.blue);
    }

    return bag;
}

fn power_of_bag(bag: &Bag) -> i32 {
    return bag.red * bag.green * bag.blue;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
