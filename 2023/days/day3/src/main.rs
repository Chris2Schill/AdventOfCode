use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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

struct Point2D
{
    row: usize,
    col: usize,
}

struct Point2(i32, i32);

struct PartNumber {
    start_idx: Point2D,
    end_idx:   Point2D,
    value: i32,
}

fn potential_part_numbers(line: &str, r: &usize) -> Vec<PartNumber> {
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    for m in re.find_iter(&line) {
        let part_num: PartNumber = PartNumber{
            start_idx: Point2D{row:r.clone(), col:m.start()},
            end_idx: Point2D{row:r.clone(), col:m.end()-1},
            value: m.as_str().parse::<i32>().unwrap()
        };
        part_numbers.push(part_num);
    }

    return part_numbers;
}

fn is_valid(point: &Point2, matrix: &[Vec<char>]) -> bool {
    return point.0 >= 0 && point.0 < matrix.len() as i32 &&
           point.1 >= 0 && point.1 < matrix[point.0 as usize].len() as i32;
}

static mut GEARS: Vec<Point2> = Vec::new();

// returns a 2d region the size of vec filled with 'y' or 'n' if a sub region can contain a valid part number
fn precompute_valid_region(schematic: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut region = vec![vec!['.'; schematic[0].len()]; schematic.len()];

    let mut row = 0;

    let mut potential_gears: Vec<Point2> = Vec::new();

    for row_data in &schematic {
        let mut col = 0;
        for &c in row_data {
            if c.is_numeric() || c == '.' {
                if region[row][col] != 'y' {
                    region[row][col] = 'n';
                }
            }
            else {
                let colc = col as i32;
                let rowc = row as i32;

                let cells: [Point2; 9] = [
                    Point2(rowc-1, colc-1), Point2(rowc-1, colc), Point2(rowc-1, colc+1),
                    Point2(rowc,   colc-1), Point2(rowc,   colc), Point2(rowc,   colc+1),
                    Point2(rowc+1, colc-1), Point2(rowc+1, colc), Point2(rowc+1, colc+1),
                ];

                for cell in cells {
                    if is_valid(&cell, &region) {
                        region[cell.0 as usize][cell.1 as usize] = 'y';
                    }
                }
                
            }

            if c == '*' {
                potential_gears.push(Point2(row as i32, col as i32));
            }

            col += 1;
        }
        row += 1;
    }

    for gear in potential_gears {
        unsafe {
            GEARS.push(gear);
        }
    }

    // for r in &region {
    //     for c in r {
    //         print!("{c}");
    //     }
    //     println!("");
    // }

    return region;
}

fn get_sum_part_numbers(potential_part_numbers: Vec<PartNumber>, schematic: Vec<Vec<char>>) -> i32 {
    let valid_region = precompute_valid_region(schematic);

    let mut sum = 0;

    for pn in &potential_part_numbers {
        let mut i = 0;

        while i < pn.value.to_string().len() {
            if valid_region[pn.start_idx.row][pn.start_idx.col+i] == 'y' {
                sum += pn.value;
                break;
            }
            i += 1;
        }
    }

    let mut sum_gear_ratio = 0;
    unsafe {
        for gear in &GEARS {
            let mut ratio = 0;
            for pn in &potential_part_numbers {
                if gear.0 >= pn.start_idx.row as i32 - 1 && gear.0 <= pn.start_idx.row as i32 + 1 && 
                   gear.1 >= pn.start_idx.col as i32 - 1 && gear.1 <= pn.end_idx.col   as i32 + 1 {
                       if ratio == 0 {
                           ratio = pn.value;
                       }
                       else {
                           ratio *= pn.value;
                           sum_gear_ratio += ratio;
                           break;
                       }
                }
            }
        }
    }
    println!("Sum gear ratios:{sum_gear_ratio}");

    return sum;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
