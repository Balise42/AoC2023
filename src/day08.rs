use regex::Regex;
use std::collections::HashMap;

fn parse_file(s: String) -> (String, HashMap::<String,(String, String)>) {
    let mut lines = s.lines();
    let instr = lines.next().unwrap().to_string();
    lines.next();
    let mut map: HashMap::<String, (String,String)> = HashMap::new();
    let re = Regex::new(r"([\dA-Z]{3}) = \(([\dA-Z]{3}), ([\dA-Z]{3})\)").unwrap();
    for line in lines {
        let caps = re.captures(line).unwrap();
        let key = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();
        map.insert(key.to_string(), (left.to_string(), right.to_string()));
    }
    return (instr, map);
}

fn get_steps(instr: String, map: HashMap::<String, (String,String)>, dep: String, arr: String) -> usize {
    let mut pos = dep;
    let mut i: usize = 0;
    let steps: Vec<char> = instr.chars().collect();
    while pos != arr {
        match steps.get(i % steps.len()).unwrap() {
            'L' => {
                pos = map.get(&pos).unwrap().0.clone();
            }
            'R' => {
                pos = map.get(&pos).unwrap().1.clone();
            },
            _ => ()
        }
        i+=1;
    }
    return i;
}

fn get_to_z(instr: String, map: HashMap::<String, (String,String)>, dep: String) -> usize {
    let mut pos = dep;
    let mut i: usize = 0;
    let steps: Vec<char> = instr.chars().collect();
    while pos.chars().last().unwrap() != 'Z' {
        match steps.get(i % steps.len()).unwrap() {
            'L' => {
                pos = map.get(&pos).unwrap().0.clone();
            }
            'R' => {
                pos = map.get(&pos).unwrap().1.clone();
            },
            _ => ()
        }
        i+=1;
    }
    return i;
}

pub fn part1(s: String) {
    let (instr, map) = parse_file(s);
    //let steps = get_steps(instr, map, "AAA".to_string(), "ZZZ".to_string());
    //println!("{}", steps);
}


// Okay, this SHOULD NOT work. It's assuming faaaar too many things
// on the input that are not actually easy to check. But, it worked out,
// and while it's definitely Not Correct wlog, it's Correct Enough for
// today.
pub fn part2(s: String) {
    let (instr, map) = parse_file(s);
    let mut numsteps = 1;
    for key in map.keys() {
        if key.chars().last().unwrap() == 'A' {
            let steps = get_to_z(instr.clone(), map.clone(), key.to_string());
            numsteps = num::integer::lcm(steps, numsteps);
        }
    }
    println!("{:?}", numsteps);
}