use std::fs;

fn main() {
    let mut sum = 0;
    let mut sequences = parse("./days/day9/day9.in");
    for seq in sequences.iter_mut() {
        println!("original = {:?}", seq);
        sum += predict(seq);
    }
    println!("Sum Predictions:{sum}");
    // let s: Vec<i32> = Vec::new();
    // predict(s);
}

fn predict(seq: &mut Vec<i32>) -> i32 {

    // base case
    let mut all_zero = true;
    seq.iter().for_each(|i| all_zero &= *i == 0 );
    if all_zero { return 0; }


    let next_seq = seq.windows(2)
                      .map(|w| w[1]-w[0])
                      .collect::<Vec<_>>();

    // let &mut next_seq_mut: &mut Vec<i32> = &next_seq.as_mut();


    print!("next = {:?}", next_seq);
    let prediction = predict2(next_seq) + seq.last().unwrap();

    println!(" -> {prediction}");
    return prediction;
}


fn predict2(seq: Vec<i32>) -> i32 {

    // base case
    let mut all_zero = true;
    seq.iter().for_each(|i| all_zero &= *i == 0 );
    if all_zero { return 0; }

    let next_seq = seq.windows(2)
                      .map(|w| w[1]-w[0])
                      .collect::<Vec<_>>();

    println!("next = {:?}", next_seq);

    let prediction = predict2(next_seq) + seq.last().unwrap();

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
