use std::fs;

fn main() {
    let sequences = parse("./days/day9/day9.in");
    let sum: i32 = sequences.into_iter().map(|seq| predict(seq)).sum();
    println!("Sum Predictions:{sum}");
}

fn predict(seq: Vec<i32>) -> i32 {

    // base case
    let mut all_zero = true;
    seq.iter().for_each(|i| all_zero &= *i == 0 );
    if all_zero { return 0; }

    let next_seq = seq.windows(2)
                      .map(|w| w[1]-w[0])
                      .collect::<Vec<_>>();

    println!("next = {:?}", next_seq);

    let prediction = predict(next_seq) + seq.last().unwrap();

    println!(" -> {prediction}");
    return prediction;
}


fn parse(file: &str) -> Vec<Vec<i32>> {

    let mut sequences: Vec<Vec<i32>> = Vec::new();

    let contents = fs::read_to_string(file).unwrap();
    let lines = contents.strip_suffix('\n').unwrap().split("\n").collect::<Vec<_>>();
    for line in lines {
        let s = line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
        sequences.push(s);
    }

    return sequences;
}
