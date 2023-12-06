use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use regex::Regex;

fn main() {

    let mut schematic = vec![vec!['#';0]; 0];
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    if let Ok(lines) = read_lines("days/day3/day3.in") {
        let mut row = 0;
        for line in lines {
            let aline = line.unwrap();
            let mut schematic_line: Vec<_> = Vec::new();

            for c in aline.chars() {
                schematic_line.push(c);
            }

            for pn in potential_part_numbers(&aline, &row) {
                part_numbers.push(pn);
            }

            schematic.push(schematic_line);
            row += 1;
        }
    }

    let sum_part_numbers = get_sum_part_numbers(part_numbers, schematic);

    println!("Sum Part Numbers:{sum_part_numbers}");
}

struct PartNumber {
    row:   usize,
    col:   usize,
    value: i32,
}

fn potential_part_numbers(line: &str, r: &usize) -> Vec<PartNumber> {
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    for m in re.find_iter(&line) {
        let part_num: PartNumber = PartNumber{
            row:r.clone(),
            col:m.start(),
            value: m.as_str().parse::<i32>().unwrap()
        };
        part_numbers.push(part_num);
    }

    return part_numbers;
}

// returns a 2d region the size of vec filled with 'y' or 'n' if a sub region can contain a valid part number
fn precompute_valid_region(schematic: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut region = vec![vec!['.'; schematic[0].len()]; schematic.len()];

    let mut row = 0;

    for row_data in schematic {
        let mut col = 0;
        for c in row_data {
            if c.is_numeric() || c == '.' {
                if region[row][col] != 'y' {
                    region[row][col] = 'n';
                }
            }
            else {
                let colc = col as i32;
                let rowc = row as i32;
                let left_of = cmp::max(0, colc-1) as usize;
                let right_of = cmp::min((region[0].len()-1) as i32, colc+1) as usize;
                let above = cmp::max(0, rowc-1) as usize;
                let below = cmp::min((region.len()-1) as i32, rowc+1) as usize;

                region[above][left_of] = 'y'; region[above][col] = 'y'; region[above][right_of] = 'y';
                region[row  ][left_of] = 'y'; region[row  ][col] = 'y'; region[row  ][right_of] = 'y';
                region[below][left_of] = 'y'; region[below][col] = 'y'; region[below][right_of] = 'y';
            }
            col += 1;
        }
        row += 1;
    }

    for r in &region {
        for c in r {
            print!("{c}");
        }
        println!("");
    }

    return region;
}

fn get_sum_part_numbers(potential_part_numbers: Vec<PartNumber>, schematic: Vec<Vec<char>>) -> i32 {
    let valid_region = precompute_valid_region(schematic);

    let mut sum = 0;

    for pn in potential_part_numbers {
        let mut i = 0;

        while i < pn.value.to_string().len() {
            if valid_region[pn.row][pn.col+i] == 'y' {
                sum += pn.value;
                break;
            }
            i += 1;
        }
    }

    return sum;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
