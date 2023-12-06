use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;


fn main() {
    let mut cards = parse("days/day4/day4.in");
    assert_eq!(cards.len(), 207);
    println!("cards len = {}", cards.len());

    let mut sum_points = 0;
    for card in &cards {
        let winners: Vec<_> = card.ours.iter().filter(|item| card.winners.contains(item)).collect();
        if winners.len() > 0 {
            let value = i32::pow(2, (winners.len()-1) as u32);
            println!("Card {}: subwinners = {:?}, value={value}", card.id, winners);
            sum_points += value;
        }
    }
    println!("Sum points:{sum_points}");

    for idx in 0..cards.len() {
        let winners: Vec<_> = cards[idx].ours.iter().filter(|item| cards[idx].winners.contains(item)).collect();
        let num_winners = winners.len();

        if winners.len() > 0 {
            for _ in 0..cards[idx].count {
                for i in 1..num_winners+1 {
                    if let Some(c) = cards.get_mut(idx+i as usize) {
                        c.count += 1;
                    }
                }
            }
        }
    }
    
    let sum_cards = cards.iter().map(|c| c.count).sum::<i32>();

    println!("Sum cards:{sum_cards}");
}

struct Card {
    id: i32,
    winners: Vec<i32>,
    ours: Vec<i32>,
    count: i32,
}

fn parse(file: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    let re = Regex::new(r"Card\s+(\d+):\s+((\d+\s+)+)+\|(.*)").unwrap();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            for (_,[id, winners, _, ours]) in re.captures_iter(&line.unwrap()).map(|c| c.extract()) {
                let mut card = Card{
                    id:id.parse::<i32>().unwrap(),
                    winners: Vec::new(),
                    ours: Vec::new(),
                    count:1,
                };

                for i in winners.split_whitespace() {
                    card.winners.push(i.parse::<i32>().unwrap());
                }

                for i in ours.split_whitespace() {
                    card.ours.push(i.parse::<i32>().unwrap());
                }

                println!("Card = {}, winners={:?}, ours={:?}", card.id, card.winners, card.ours);
                cards.push(card);
            }
        }
    }

    return cards;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
