use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time::Duration};

fn print_grid(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|c| {
            print!("{c}");
        });
        println!("");
    });
    println!("");
}

fn main() {

    let mut grid = parse("./days/day10/day10.in").unwrap();
    print_grid(&grid);

    let mut paths = find_start(&grid).unwrap();

    grid[1][1] = '*';
    print_grid(&grid);

    let mut steps = 1;

    while paths[0].x != paths[1].x || paths[0].y != paths[1].y {
        step(&mut paths[0], &mut grid);
        step(&mut paths[1], &mut grid);
        // thread::sleep(Duration::from_millis(10));

        steps += 1;
    }

    print_grid(&grid);
    println!("Steps:{steps}");
}

#[derive(PartialEq)]
enum Direction { Up, Down, Left, Right }

struct Pipe {
    x: usize,
    y: usize,
    dir: Direction,
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<Vec<Pipe>> {
    for (y,row) in grid.iter().enumerate() {
        for (x,c) in row.iter().enumerate() {
            if c == &'S' {

                println!("found S ({x},{y})");
                let mut paths: Vec<Pipe> = vec![];

                if is_connected(&x,&y, Direction::Up, &grid) {
                    paths.push(Pipe{x,y:y-1, dir: Direction::Up});
                }
                if is_connected(&x,&y, Direction::Down, &grid) {
                    paths.push(Pipe{x,y:y+1, dir: Direction::Down});
                }
                if is_connected(&x,&y, Direction::Left, &grid) {
                    paths.push(Pipe{x:x-1,y, dir: Direction::Left});
                }
                if is_connected(&x,&y, Direction::Right, &grid) {
                    paths.push(Pipe{x:x+1,y, dir: Direction::Right});
                }
                assert_eq!(paths.len(), 2);

                return Some(paths);
            }
        }
    }
    None
}

fn is_connected(x: &usize, y: &usize, dir: Direction, grid: &Vec<Vec<char>>) -> bool {
    match dir {
        Direction::Up    => { 
            match grid[y-1][*x] {
                '|' => return true,
                '7' => return true,
                'F' => return true,
                 _  => return false,
            }
        },
        Direction::Down  => {
            match grid[y+1][*x] {
                '|' => return true,
                'J' => return true,
                'L' => return true,
                 _  => return false,
            }
        },
        Direction::Left  => {
            match grid[*y][x-1] {
                '-' => return true,
                'F' => return true,
                'L' => return true,
                 _  => return false,
            }
        },
        Direction::Right => {
            match grid[*y][x+1] {
                '-' => return true,
                'J' => return true,
                '7' => return true,
                 _  => return false,
            }
        },
    }
}


fn step(s: &mut Pipe, grid: &mut Vec<Vec<char>>){
    let curr = grid[s.y][s.x];
    grid[s.y][s.x] = '*';
    match curr {
        '|' => {
            match s.dir {
                Direction::Up   => { s.y -= 1},
                Direction::Down => { s.y += 1},
                _ => {},
            }
        },
        '-' => {
            match s.dir {
                Direction::Left  => { s.x -= 1},
                Direction::Right => { s.x += 1},
                _ => {},
            }
        },
        'F' => {
            match s.dir {
                Direction::Up   => { s.x += 1; s.dir = Direction::Right; },
                Direction::Left => { s.y += 1; s.dir = Direction::Down; },
                _ => {},
            }
        },
        'L' => {
            match s.dir {
                Direction::Down => { s.x += 1; s.dir = Direction::Right; },
                Direction::Left => { s.y -= 1; s.dir = Direction::Up; },
                _ => {},
            }
        },
        'J' => {
            match s.dir {
                Direction::Down  => { s.x -= 1; s.dir = Direction::Left; },
                Direction::Right =>  { s.y -= 1; s.dir = Direction::Up; },
                _ => {},
            }
        },
        '7' => {
            match s.dir {
                Direction::Up  =>   { s.x -= 1; s.dir = Direction::Left; },
                Direction::Right => { s.y += 1; s.dir = Direction::Down; },
                _ => {},
            }
        },
         _ => {},
    }
}

fn parse(file: &str) -> Option<Vec<Vec<char>>> {
    if let Ok(lines) = read_lines(file) {
        return Some(lines.map(|l| l.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>());
    }
    return None;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
