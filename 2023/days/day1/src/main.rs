use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum_part_one = 0;
    let mut sum_part_two = 0;
    if let Ok(lines) = read_lines("days/day1/day1.in") {
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
        if c.is_numeric() {
            value += c.to_string().parse::<i32>().unwrap()*10;
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_numeric() {
            value += c.to_string().parse::<i32>().unwrap();
            break;
        }
    }
    return value;
}


fn value_part_two(line: &str) -> i32 {
    const NUM_TEXT: [&str; 19] = [
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

    let mut i = 0;
    let mut nums = vec![];

    for _c in line.chars() {
        for num_text in NUM_TEXT {
            let len = num_text.len();

            if i+len > line.len() {
                continue;
            }

            let slice = &line[i..(i+len)];
            if slice == num_text {
                let mut idx = NUM_TEXT.into_iter().position(|x| x == slice.to_string()).unwrap() as i32;
                if idx > 9 {
                    idx -= 9;
                }
                nums.push(idx);
                break;
            }
        }
        i += 1;
    }

    let value = nums.first().unwrap()*10 +
                nums.last().unwrap();
    // println!("final value for {line} = {value}");
    return value;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
