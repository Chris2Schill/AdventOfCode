use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let (map,branches) = parse("./days/day8/day8.in");
    let char_map = generate_char_to_int_mapping();
    
    // part1
    let mut curr = char_map["AAA"];
    let goal = char_map["ZZZ"];
    let mut steps = 0;
    while curr != goal {
        for c in map.directions.chars() {
            match c {
                'L' => {
                    curr = map.nodes[&curr].u_left;
                    steps += 1;
                },
                'R' => {
                    curr = map.nodes[&curr].u_right;
                    steps += 1;
                },
                 _  => {},
            }
        }
    }
    println!("Steps (Part 1):{steps}");

    // part2
    steps = 0;
    let mut quit = false;
    while !quit {
        for c in map.directions.chars() {
            let mut zs = 0;

            fn is_z(m: &Map, n: &usize) -> usize {
                if m.nodes[&n].last_char == 'Z' {
                    return 1;
                }
                return 0;
            }


            for mut b in &branches {
                match c {
                    'L' => {
                        b = &map.nodes[&b].u_left;
                        zs += is_z(&map, b);
                    },
                    'R' => {
                        b = &map.nodes[&b].u_right;
                        zs += is_z(&map, b);
                    },
                    _  => {},
                }
            }
            steps += 1;

            if zs == branches.len() {
                quit = true;
                println!("Steps (Part 2):{steps}");
                break;
            }
        }
    }

    println!("Steps (Part 2):{steps}");
    println!("branches.len={}", branches.len());
}

struct Node {
    u_left: usize,
    u_right: usize,

    last_char: char,
}
type CharMap = HashMap<String, usize>;
type NodesMap = HashMap<usize, Node>;

fn print_char_value_pair(s: &str, char_to_int_map: &CharMap) {
    println!("{}={}", s, char_to_int_map[s]);
}

type Branch = usize;

struct Map {
    directions: String,
    nodes: NodesMap,
}

const CHARS: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

fn generate_char_to_int_mapping() -> CharMap {

    let mut map: HashMap<String, usize> = HashMap::new();

    for (i,a) in CHARS.iter().enumerate() {
        for (j,b) in CHARS.iter().enumerate() {
            for (k,c) in CHARS.iter().enumerate() {
                let mut value_str = String::new();
                value_str.push_str(&(k+1).to_string());
                value_str.push_str(&(j+1).to_string());
                value_str.push_str(&(i+1).to_string());
                let value = value_str.parse::<usize>().unwrap();

                let mut str = String::new();
                str.push_str(&c.to_string());
                str.push_str(&b.to_string());
                str.push_str(&a.to_string());

                map.insert(str, value);
            }
        }
    }

    return map;
}

fn parse(file: &str) -> (Map, Vec<Branch>) {

    let mut nodes: NodesMap = NodesMap::new();
    let mut branches: Vec<Branch> = Vec::new();

    let char_map = generate_char_to_int_mapping();

    let contents = fs::read_to_string(file).unwrap();
    let lines = contents.split("\n").collect::<Vec<_>>();
    for line in &lines {
        let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();

        for (_,[key, left, right]) in re.captures_iter(line).map(|c| c.extract()) {
            println!("[{key},{left},{right}]");
            let k = char_map[key];
            let node: Node = Node{
                u_left:  char_map[left],
                u_right: char_map[right],
                last_char: key.chars().nth(2).unwrap(),
            };
            nodes.insert(k, node);

            if key.chars().nth(2).unwrap() == 'A' {
                branches.push(k); 
            }
        }
    }
    println!("firstline={}", lines.get(0).unwrap());
    let directions = lines.get(0).unwrap().to_string();

    return (Map{directions, nodes}, branches);
}
