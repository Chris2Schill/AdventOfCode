use std::fs;
use std::fmt;
use std::cmp::Ordering;

fn main() {
    let mut hands = parse("./days/day7/day7.in");
    hands.sort_by(|a,b| {
        cmp_hand_strength(a,b)
    });
    let mut total_winnings = 0;
    hands.iter().rev().enumerate().for_each(|(i,h)| total_winnings += (i as i32 +1)*h.bid );//println!("rank:{}, bid:{}", i, h.bid)).collect::<Vec<_>>();
    println!("Total Winnings:{total_winnings}");
    // println!("{:?}", hands);
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse   = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

fn card_value(c: &char) -> Option<usize> {
    match c {
        '2' =>  Some(0),
        '3' =>  Some(1),
        '4' =>  Some(2),
        '5' =>  Some(3),
        '6' =>  Some(4),
        '7' =>  Some(5),
        '8' =>  Some(6),
        '9' =>  Some(7),
        'T' =>  Some(8),
        'J' =>  Some(9),
        'Q' =>  Some(10),
        'K' =>  Some(11),
        'A' =>  Some(12),
        _   =>  None,
    }
}

fn classify(hand: &str) -> HandType {
    let mut cards: [i32; 13] = [0;13];

    for c in hand.chars() {
        if let Some(v) = card_value(&c) {
            cards[v] += 1;
        }
    }

    let mut has_five = false;
    let mut has_four = false;
    let mut has_three = false;
    let mut pairs = 0;

    for c in cards {
        match c {
            5 => { has_five = true; },
            4 => { has_four = true; },
            3 => { has_three = true;},
            2 => { pairs += 1},
            _ => {},
        }
    }

    // println!("{:?}", cards);

    if has_five {
        return HandType::FiveOfAKind;
    }
    else if has_four {
        return HandType::FourOfAKind;
    }
    else if has_three && pairs == 1 {
        return HandType::FullHouse;
    }
    else if has_three {
        return HandType::ThreeOfAKind;
    }
    else if pairs == 2 {
        return HandType::TwoPair;
    }
    else if pairs == 1 {
        return HandType::OnePair;
    }
    else if pairs == 1 {
        return HandType::OnePair;
    }
    else {
        return HandType::HighCard;
    }
}

// return less if left should come before right
fn cmp_hand_strength(left: &Hand, right: &Hand) -> Ordering {

    if left.hand_type as i32 == right.hand_type as i32 {
        for i in 0..5 {
            let v_left = card_value(&left.hand.chars().nth(i).unwrap());
            let v_right = card_value(&right.hand.chars().nth(i).unwrap());
            if v_left > v_right {
                return Ordering::Less;
            }
            else if v_left < v_right {
                return Ordering::Greater
            }
        }
    }

    if left.hand_type as i32 > right.hand_type as i32 {
        return Ordering::Less;
    }

    return Ordering::Greater;
}


#[derive(Debug)]
struct Hand
{
    hand: String,
    bid: i32,
    hand_type: HandType,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hand:{}, bid:{}", self.hand, self.bid)
    }
}

fn parse(file: &str) -> Vec<Hand> {
    let contents = fs::read_to_string(file).unwrap();
    let mut hands = Vec::new();

    for line in contents.split('\n').into_iter() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        if tokens.len() == 2 {
            hands.push(Hand {
                hand: tokens[0].to_string(),
                bid: tokens[1].parse::<i32>().unwrap(),
                hand_type: classify(tokens[0]),
            });
        }
    }

    return hands;
}
