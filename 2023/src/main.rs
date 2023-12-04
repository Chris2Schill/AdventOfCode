use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::RegexSet;


fn main() {

    // let num_regex = r"(one|two|three|four|five|six|seven|eight|nine)*";
    let set = RegexSet::new(&[
        r"zero",
        r"one",
        r"two",
        r"three",
        r"four",
        r"five",
        r"six",
        r"seven",
        r"eight",
        r"nine",
    ]).unwrap();

    let mut sum = 0;
    if let Ok(lines) = read_lines("./day1.in") {
        for line in lines {

            let mut value = 0;

            for c in line.as_ref().expect("Chars").chars() {
                if c.is_digit(10) {
                    // println!("{}", char);
                    value += c.to_string().parse::<i32>().unwrap()*10;
                    break;
                }
            }

            for c in line.as_ref().expect("Chars").chars().rev() {
                if c.is_digit(10) {
                    // println!("{}", char);
                    value += c.to_string().parse::<i32>().unwrap();
                    break;
                }
            }
            sum += value;
            // println!("{line} | {value}");

            // let re = Regex::new(num_regex).unwrap();

            // let Some(caps) = re.captures(line.as_ref().expect("AString")) else {
            //     println!("no match");
            //     break;
            // };


            let aline = line.as_ref().expect("A string").to_string();
            // let caps = &re.captures_iter(&aline).count();

            let matches: Vec<_> = set.matches(&aline).into_iter().collect();
            let num_matches = &matches.len();

            println!("{aline} | num_matches={num_matches}");

            // for m in &matches {
            //     println!("match={m}");
            // }
            //
            value += matches[0].to_string().parse::<i32>().unwrap()*10;
            value += matches[&matches.len()-1].to_string().parse::<i32>().unwrap();
            sum += value;
            // re.find_iter(s) {
            //     let str = cap.as_str();
            //     println!("cap={str}");
            // }
            // println!("count = {}", caps);
            // for caps in re.captures_iter(&num_regex) {

            // println!("firstcap = {:?}", caps.get(2).unwrap().as_str());
            // println!("lastcap = {}", caps.get(caps.len()).unwrap().as_str());

            // }


            // for cap in re.captures_iter("[S]") {
            // }

            let line = line.as_ref().expect("line");



        }
    }


    println!("Sum:\n{sum}");

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
