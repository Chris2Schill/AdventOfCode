use std::fs;

fn main() {
    let races = parse("./days/day6/day6.in");
    assert_eq!(races.times.len(), races.distances.len());

    for (time,distance) in races.times.iter().zip(races.distances.iter()) {
        println!("time={time}, distances={distance}");
        permute(*time, *distance);
    }

    // Part 1
    let mut product_win_count = 1;
    races.times.iter().zip(races.distances.iter()).for_each(|(time,distance)| product_win_count *= permute(*time, *distance));
    println!("Product win count:{product_win_count}");

    // Part 2
    let mut time_str: String = String::new();
    let mut distance_str: String = String::new();
    races.times.iter().for_each(|t| time_str.push_str(&t.to_string()));
    races.distances.iter().for_each(|d| distance_str.push_str(&d.to_string()));
    let win_count = permute(
        time_str.parse::<u64>().unwrap(),
        distance_str.parse::<u64>().unwrap());
    println!("Win count:{win_count}");
}

struct Races {
    times: Vec<u64>,
    distances: Vec<u64>,
}

fn permute(mut time: u64, record: u64) -> u64 {
    
    let mut speed = 0;
    let accel= 1;
    let time_step = 1;

    let mut win_count = 0;

    while time != 0 {
        let distance = speed * time;
        if distance > record {
            win_count += 1;
        }

        time  -= time_step;
        speed += accel*time_step;
    }

    return win_count;
}


fn parse(file: &str) -> Races {
    let contents = fs::read_to_string(file).unwrap();
    let lines = contents.split('\n').collect::<Vec<_>>();

    let times = lines[0].split(':').collect::<Vec<_>>()[1].split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let distances = lines[1].split(':').collect::<Vec<_>>()[1].split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    return Races{
        times: times.clone(),
        distances: distances.clone(),
    };
}
