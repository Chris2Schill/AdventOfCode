use std::fs;
use std::fmt;
use std::cmp::Ordering;

fn main() {
    let mut hands = parse("./days/day7/day7.in");

    // Part 1
    hands.sort_by(|a,b| {
        cmp_hand_strength(&a,&b)
    });
    fn sum_winnings(hands: &Vec<Hand>) -> i32 {
        let mut sum = 0;
        hands.iter().rev().enumerate().for_each(|(i,h)| sum += ((i+1) as i32) * h.bid );
        return sum;
    }
    let total_winnings = sum_winnings(&hands);
    println!("Total Winnings:{total_winnings}");


    // Part 2
    for i in 0..hands.len() {
        let new_hand_type = collapse_joker_wavefunction(&hands[i]);
        if let Some(ht) = new_hand_type {
            hands[i].hand_type = ht;
        }
    }
    hands.sort_by(|a,b| {
        cmp_hand_strength_jokers_weakest(&a,&b)
    });
    let total_winnings2 = sum_winnings(&hands);
    // for h in hands.iter().rev() {
    //     println!("hand:{}, type={}, bid={}", h.hand, h.hand_type as i32, h.bid);
    // }
    println!("Total Winnings2:{total_winnings2}");
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

fn card_value(c: char) -> Option<usize> {
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

fn card_value_jokers_weakest(c: char) -> Option<usize> {
    match c {
        'J' =>  Some(0),
        '2' =>  Some(1),
        '3' =>  Some(2),
        '4' =>  Some(3),
        '5' =>  Some(4),
        '6' =>  Some(5),
        '7' =>  Some(6),
        '8' =>  Some(7),
        '9' =>  Some(8),
        'T' =>  Some(9),
        'Q' =>  Some(10),
        'K' =>  Some(11),
        'A' =>  Some(12),
        _   =>  None,
    }
}

fn collapse_joker_wavefunction(hand: &Hand) -> Option<HandType> {

    let mut cards: [i32; 13] = [0;13];

    for c in hand.hand.chars() {
        if let Some(v) = card_value_jokers_weakest(c) {
            cards[v] += 1;
        }
    }

    // never collapse jokers to J
    cards[card_value_jokers_weakest('J').unwrap()] = 0;

    let mut highest_max_idx = cards.len()-1;
    let mut max_cards = 0;

    for i in (0..cards.len()).rev() {
        if cards[i] > max_cards {
            max_cards = cards[i];
            highest_max_idx = i;
        }
    }

    fn card_from_idx(idx: usize) -> Option<char> {
        match idx {
            0  => { Some('J') },
            1  => { Some('2') },
            2  => { Some('3') },
            3  => { Some('4') },
            4  => { Some('5') },
            5  => { Some('6') },
            6  => { Some('7') },
            7  => { Some('8') },
            8  => { Some('9') },
            9  => { Some('T') },
            10 => { Some('Q') },
            11 => { Some('K') },
            12 => { Some('A') },
            _  => { None },
        }
    }

    let replacing_card = card_from_idx(highest_max_idx);
    if let Some(c) = replacing_card {
        let collapsed_hand = str::replace(&hand.hand,"J", &c.to_string());
        // println!("hand:{} | highest_max_idx={highest_max_idx},amnt={max_cards}, collapsed_hand={collapsed_hand}", hand.hand);
        return Some(classify(&collapsed_hand));
    }

    return Some(hand.hand_type);
}

fn classify(hand: &str) -> HandType {

    let mut cards: [i32; 13] = [0;13];

    for c in hand.chars() {
        if let Some(v) = card_value(c) {
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

    let l_type = left.hand_type as i32;
    let r_type = right.hand_type as i32;

    if l_type == r_type{
        for i in 0..5 {
            let v_left = card_value(left.hand.chars().nth(i).unwrap());
            let v_right = card_value(right.hand.chars().nth(i).unwrap());
            if v_left > v_right {
                // println!("left={left}, right={right} left > right");
                return Ordering::Less;
            }
            else if v_left < v_right {
                // println!("left={left}, right={right} left < right");
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }

    if l_type > r_type {
        return Ordering::Less;
    }

    return Ordering::Greater;
}

fn cmp_hand_strength_jokers_weakest(left: &Hand, right: &Hand) -> Ordering {

    let l_type = left.hand_type as i32;
    let r_type = right.hand_type as i32;

    if l_type == r_type{
        for i in 0..5 {
            let v_left = card_value_jokers_weakest(left.hand.chars().nth(i).unwrap());
            let v_right = card_value_jokers_weakest(right.hand.chars().nth(i).unwrap());
            if v_left > v_right {
                // println!("left={left}, right={right} left > right");
                return Ordering::Less;
            }
            else if v_left < v_right {
                // println!("left={left}, right={right} left < right");
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }

    if l_type > r_type {
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
