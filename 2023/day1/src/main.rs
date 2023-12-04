use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::RegexSet;

fn main() {
    let mut sum_part_one = 0;
    let mut sum_part_two = 0;
    if let Ok(lines) = read_lines("./day1.in") {
        for line in lines {
            let aline = &line.unwrap();
            sum_part_one += value_part_one(aline);
            sum_part_two += value_part_two(aline);
        }
    }

    println!("Sum (Part 1):{sum_part_one}\nSum (Part 2):{sum_part_two}");

}

fn value_part_one(line: &str) -> i32 {
    let mut value = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            value += c.to_string().parse::<i32>().unwrap()*10;
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_digit(10) {
            value += c.to_string().parse::<i32>().unwrap();
            break;
        }
    }
    return value;
}

fn value_part_two(line: &str) -> i32 {


    fn find_first_digit(line: &str) -> i32 {
        const NUMS: [&str; 19] = [
            "zero",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
        ];
        let set = RegexSet::new(NUMS).unwrap();
        let mut matches: Vec<_> = set.matches(line).into_iter().map(|match_idx| &set.patterns()[match_idx]).collect();
        matches.sort_by(|a,b| line.find(a.as_str()).unwrap().cmp(&line.find(b.as_str()).unwrap()));
        let ordered: Vec<_> = matches.iter().map(|e| NUMS.into_iter().position(|x| x == e.to_string())).collect();
        let mut digit = ordered[0].unwrap();
        if digit > 9 {
            digit -= 9;
        }
        return digit as i32;
    }

    fn find_second_digit(line: &str) -> i32 {
        const NUMS_REV: [&str; 19] = [
            "orez",
            "eno",
            "owt",
            "eerht",
            "ruof",
            "evif",
            "xis",
            "neves",
            "thgie",
            "enin",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
        ];

        let set_rev = RegexSet::new(NUMS_REV).unwrap();
        let line_rev = line.chars().rev().collect::<String>();
        let mut matches_rev: Vec<_> = set_rev.matches(&line_rev).into_iter().map(|match_idx| &set_rev.patterns()[match_idx]).collect();
        matches_rev.sort_by(|a,b| line_rev.find(a.as_str()).unwrap().cmp(&line_rev.find(b.as_str()).unwrap()));
        let ordered_rev: Vec<_> = matches_rev.iter().map(|e| NUMS_REV.into_iter().position(|x| x == e.to_string())).collect();
        let mut digit = ordered_rev[0].unwrap();
        if digit > 9 {
            digit -= 9;
        }
        return digit as i32;
    }

    return find_first_digit(line)*10 + find_second_digit(line);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
