use std::fs;
use std::fmt;
use std::collections::HashMap;

fn main() {
    let alminac = parse("days/day5/day5.in");

    // part 1
    let min_location = alminac.seeds.iter()
                                    .map(|s| location(*s, &alminac))
                                    .collect::<Vec<_>>()
                                    .into_iter()
                                    .min();

    println!("min location:{}", min_location.unwrap_or(0));


    // Part 2
    let mut locations_cache: HashMap<i64, i64> = HashMap::new();
    let mut min_location2: i64 = i64::MAX;
    alminac.seeds.chunks(2)
                 .map(|s|  [s[0],s[1]])
                 .for_each(|[s,r]| {
                     // println!("{s},{e}");
                     // let l = (s..s+e+1)
                     //     .map(|j| location(j, &alminac))
                     //     .collect::<Vec<_>>()
                     //     .into_iter()
                     //     .min().unwrap();

                     for i in s..s+r+1 {
                         let loc = locations_cache.get(&i);
                         match loc {
                             Some(_) => {},
                             None => { 
                                 let new_loc = location(i, &alminac);
                                 locations_cache.insert(i, new_loc);
                                 min_location2 = i64::min(new_loc, min_location2);
                             }
                         }
                     }
                 });
    println!("min location2:{min_location2}");
}

fn location(seed: i64, alminac: &Alminac) -> i64 {
    let soil = map_value(&alminac.seed_to_soil, seed);
    let fert = map_value(&alminac.soil_to_fertilizer, soil);
    let water = map_value(&alminac.fertilizer_to_water, fert);
    let light = map_value(&alminac.water_to_light, water);
    let temp = map_value(&alminac.light_to_temp, light);
    let humidity = map_value(&alminac.temp_to_humidity, temp);
    let location = map_value(&alminac.humidity_to_location, humidity);
    return location;
}

fn parse(file: &str) -> Alminac
{
    fn parse_section(section: &str) -> Vec<Entry> {
        return section.split(':')
                  .collect::<Vec<_>>()[1]
                  .split('\n')
                  .filter_map(|s| {
                      let ns = s.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
                      if ns.len() == 3 {
                          Some(Entry {
                              dest_start: ns[0],
                              src_start: ns[1],
                              range: ns[2],
                              min_index: i64::MAX,
                          })
                      }
                      else {
                          None
                      }
                  }).collect::<Vec<_>>();
    }

    let contents = fs::read_to_string(file).unwrap();
    let sections: Vec<_> = contents.split("\n\n").collect();

    let seeds = sections[0].split(':')
                       .collect::<Vec<_>>()[1]
                       .split_whitespace()
                       .map(|s| s.parse::<i64>().unwrap())
                       .collect::<Vec<_>>();

    println!("seeds: {:?}", seeds);

    let seed_to_soil = parse_section(sections[1]);
    println!("seed to soils: {:?}", seed_to_soil);

    let soil_to_fertilizer = parse_section(sections[2]);
    println!("soil to fertilizer: {:?}", soil_to_fertilizer);

    let fertilizer_to_water = parse_section(sections[3]);
    println!("fertilizer to water: {:?}", fertilizer_to_water);

    let water_to_light = parse_section(sections[4]);
    println!("water_to_light: {:?}", water_to_light);

    let light_to_temp = parse_section(sections[5]);
    println!("light_to_temp: {:?}", light_to_temp);

    let temp_to_humidity = parse_section(sections[6]);
    println!("temp_to_humidity: {:?}", temp_to_humidity);

    let humidity_to_location = parse_section(sections[7]);
    println!("humidity_to_location: {:?}", humidity_to_location);

    return Alminac{
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    };
}

fn map_value(entries: &Vec<Entry>, value: i64) -> i64 {

    for e in entries {
        let offset = e.dest_start - e.src_start;
        if e.src_start <= value && value <= e.src_start + e.range {
            return value + offset;
        }
    }

    return value;
}

#[derive(Debug)]
struct Entry {
    dest_start: i64,
    src_start: i64,
    range: i64,
    min_index: i64,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.dest_start, self.src_start, self.range)
    }
}


struct Alminac {
    seeds: Vec<i64>,
    seed_to_soil: Vec<Entry>,
    soil_to_fertilizer: Vec<Entry>,
    fertilizer_to_water: Vec<Entry>,
    water_to_light: Vec<Entry>,
    light_to_temp: Vec<Entry>,
    temp_to_humidity: Vec<Entry>,
    humidity_to_location: Vec<Entry>,
}
