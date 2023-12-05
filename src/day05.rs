use std::collections::HashMap;

#[derive(Debug, Clone)]
struct SDInterval {
    source: u64,
    dest: u64,
    len: u64,
}

fn parse_ints( s: &str ) -> Vec<u64> {
    s.split_whitespace()
    .map(|x| x.parse::<u64>().expect("seed"))
    .collect()
}

fn parse_ints_to_sdinterval( s: &str ) -> SDInterval {
    let nums = parse_ints(s);
    SDInterval {
        source: *nums.get(1).unwrap(),
        dest: *nums.get(0).unwrap(),
        len: *nums.get(2).unwrap(),
    }
}

fn parse_almanac(s: String) -> (Vec::<u64>, HashMap::<String, Vec::<SDInterval>>) {
    let mut seeds: Vec::<u64> = Vec::new(); 
    let mut intervals: HashMap::<String, Vec::<SDInterval>> = HashMap::new();
    let mut next_map = "";
    for line in s.lines() {
        if line.starts_with("seeds") {
            seeds = parse_ints(line.strip_prefix("seeds: ").unwrap());
        } else if line.len() == 0 {
            continue;
        } else if line.chars().next().unwrap().is_numeric() {
            intervals.entry(next_map.to_string()).or_insert(Vec::new()).push(parse_ints_to_sdinterval(line));
        } else {
            next_map = line;
        }
    }
    return (seeds, intervals);
}

fn get_seed_position(seed: u64, intervals: HashMap::<String, Vec::<SDInterval>>) -> u64 {
    let soil = get_position(seed, intervals.get("seed-to-soil map:").unwrap());
    let fert = get_position(soil, intervals.get("soil-to-fertilizer map:").unwrap());
    let water = get_position(fert, intervals.get("fertilizer-to-water map:").unwrap());
    let light = get_position(water, intervals.get("water-to-light map:").unwrap());
    let temp = get_position(light, intervals.get("light-to-temperature map:").unwrap());
    let humidity = get_position(temp, intervals.get("temperature-to-humidity map:").unwrap());
    let loc = get_position(humidity, intervals.get("humidity-to-location map:").unwrap());
    return loc;
}

fn get_position(seed: u64, intervals: &Vec::<SDInterval>) -> u64 {
    for interval in intervals {
        if seed >= interval.source && seed < interval.source + interval.len {
            return interval.dest + (seed - interval.source);
        }
    }
    return seed;
}

pub fn part1(s: String) {
    let (seeds, intervals) = parse_almanac(s);
    let mut min_seed = 0;
    let mut min_pos = std::u64::MAX;
    for seed in seeds {
        let pos = get_seed_position(seed, intervals.clone());
        if pos < min_pos {
            min_seed = seed;
            min_pos = pos;
        }
    }
    println!("{}", min_pos);
}

#[derive(Debug, Clone)]
struct Interval {
    start: u64,
    len: u64
}

impl Interval {
    fn intersect(&self, sd: &SDInterval) -> Option::<Interval> {
        if (self.start < sd.source && self.start + self.len < sd.source) || (sd.source < self.start && sd.source + sd.len < self.start) {
            return None;
        }
        let low = core::cmp::max(sd.source, self.start);
        let high = core::cmp::min(sd.source + sd.len, self.start + self.len);
        return Some(Interval{start: low, len: high-low});
    }

    fn get_covered_intervals(&self, sdintervals: &Vec<SDInterval>) -> (Vec::<Interval>, Vec::<Interval>) {
        let mut source:Vec::<Interval> = Vec::new();
        let mut dest:Vec::<Interval> = Vec::new();
        for sd in sdintervals {
            match self.intersect(sd) {
                Some(a) => {
                    dest.push(Interval{start: sd.dest + a.start - sd.source , len: a.len});
                    source.push(a);
                },
                None => ()
            }
        }
        return (source, dest);
    }

    fn get_uncovered_intervals(&self, intervals: &Vec<Interval>) -> Vec::<Interval> {
        let mut res: Vec::<Interval> = Vec::new();
        if intervals.len() == 0 {
            res.push(self.clone());
            return res;
        }
        if self.start < intervals.get(0).expect("0").start {
            res.push(Interval{start: self.start, len: intervals.get(0).expect("0").start - self.start});
        }
        for i in 0..intervals.len()-1 {
            let a = intervals.get(i).expect("first");
            let b = intervals.get(i+1).expect("second");
            if a.start + a.len < b.start {
                res.push(Interval{start: a.start + a.len, len: b.start - 1 - (a.start + a.len - 1)});
            }
        }
        let last = intervals.get(intervals.len()-1).expect("last");
        if self.start + self.len -1 > last.start + last.len - 1 {
            res.push(Interval{start:last.start + last.len, len: self.start + self.len - (last.start + last.len)});
        }
        return res;
    }

    fn map_to(&self, sdintervals: &Vec<SDInterval>) -> Vec::<Interval> {
        let (mut source, mut dest) = self.get_covered_intervals(sdintervals);
        source.sort_by_key(|x| x.start);
        dest.append(&mut self.get_uncovered_intervals(&source));
        return dest;
    }
}

#[test]
fn test_map_to() {
    let a = Interval{start: 4, len: 20};
    let mut vec:Vec::<SDInterval> = Vec::new();
    vec.push(SDInterval{source: 8, len: 3, dest:42});
    vec.push(SDInterval{source:15, len: 2, dest:90});
    vec.push(SDInterval{source: 20, len: 1, dest: 200});
    println!("{:?}", a.map_to(&vec));
}

fn get_min_pos(a: Interval, maps: HashMap<String, Vec<SDInterval>>) -> u64 {
    let mut minpos = std::u64::MAX;
    let soils = a.map_to(maps.get("seed-to-soil map:").expect("seedtosoilmap"));
    for soil in soils {
        let ferts = soil.map_to(maps.get("soil-to-fertilizer map:").expect("soiltofert"));
        for fert in ferts {
            let waters = fert.map_to(maps.get("fertilizer-to-water map:").expect("ferttowater"));
            for water in waters {
                let lights = water.map_to(maps.get("water-to-light map:").expect("watertolight"));
                for light in lights{
                    let temps = light.map_to(maps.get("light-to-temperature map:").expect("lighttotemp"));
                    for temp in temps {
                        let humidities = temp.map_to(maps.get("temperature-to-humidity map:").expect("temptohumid"));
                        for humidity in humidities {
                            let mut positions = humidity.map_to(maps.get("humidity-to-location map:").expect("humtopos"));
                            positions.sort_by_key(|x| x.start);
                            if positions.len() > 0 {
                                minpos = core::cmp::min(minpos, positions.get(0).expect("position").start);
                            }
                        }
                    }
                }
                    
            }
        }
    }
    return minpos;
}

pub fn part2(s: String) {
    let (seeds, intervals) = parse_almanac(s);
    let mut minpos = std::u64::MAX;
    let mut i = 0;
    while i < seeds.len() {
        let mut a = Interval{start: *(seeds.get(i).expect("even")), len: *(seeds.get(i+1).expect("odd"))};
        minpos = core::cmp::min(minpos, get_min_pos(a, intervals.clone()));
        i += 2;
    }
    println!("{}", minpos);
}