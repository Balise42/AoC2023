use std::collections::HashMap;
use regex::Regex;

#[derive(Clone, Debug)]
struct Rule {
    cat: char,
    lt: bool,
    val: i32,
    dest: String,
}

fn piece_value(p: HashMap::<char, i32>) -> i32 {
    p.get(&'x').expect("x") + p.get(&'m').expect("m") + p.get(&'a').expect("a") + p.get(&'s').expect("s")
}

fn process_workflow(p: &HashMap::<char, i32>, workflows: HashMap::<String, Vec::<Rule>>, name: String) -> String {
    if (name == "A" || name == "R" ) {
        return name;
    }
    if *p.get(&'x').unwrap() > 4000 || *p.get(&'m').unwrap() > 4000 || *p.get(&'a').unwrap() > 4000 || *p.get(&'s').unwrap() > 4000 {
        return "R".to_string();
    }
    let wf = workflows.get(&name).expect("workflow");
    for rule in wf {
        if rule.cat == '.' {
            return process_workflow(p, workflows.clone(), rule.dest.clone());
        }
        if rule.lt &&  p.get(&rule.cat).expect("val") < &rule.val {
            return process_workflow(p, workflows.clone(), rule.dest.clone());
        }
        if !rule.lt && p.get(&rule.cat).expect("val") > &rule.val {
            return process_workflow(p, workflows.clone(), rule.dest.clone());
        }
    }
    return "_".to_string();
}

fn parse_piece(line: &str) -> HashMap::<char, i32> {
    let mut piece: HashMap::<char, i32> = HashMap::new();
    let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let caps = re.captures(line).unwrap();
    piece.insert('x', caps.get(1).unwrap().as_str().parse::<i32>().unwrap());
    piece.insert('m', caps.get(2).unwrap().as_str().parse::<i32>().unwrap());
    piece.insert('a', caps.get(3).unwrap().as_str().parse::<i32>().unwrap());
    piece.insert('s', caps.get(4).unwrap().as_str().parse::<i32>().unwrap());
    return piece;
}

fn parse_workflow(line: &str) -> (String, Vec::<Rule>, HashMap::<char, Vec::<i32>>) {
    let mut wf: Vec::<Rule> = Vec::new();
    let mut bounds: HashMap::<char, Vec::<i32>> = HashMap::new();

    let reA = Regex::new(r"([xmas][<>]\d+:A,A\})").unwrap();
    let reR = Regex::new(r"([xmas][<>]\d+:R,R\})").unwrap();

    let mut l = line.to_string();
    while(reA.is_match(&l) || reR.is_match(&l)) {
        let mut bindA = reA.replace(&l, "A}");
        l = bindA.to_mut().clone();
        let mut bindR = reR.replace(&l, "R}");
        l = bindR.to_mut().clone();
    }

    let mut rule = Rule {cat: '.', lt: false, val: 0, dest: "".to_string()};
    let mut buf: Vec<char> = Vec::new();
    let mut name: String = "".to_string();
    
    let mut started = false;
    for c in l.chars() {
        match c {
            '{' => { started = true; },
            'a' => { if started { buf.push(c); } else {name.push(c)}; },
            'x' => { if started { buf.push(c); } else {name.push(c)}; },
            'm' => { if started { buf.push(c); } else {name.push(c)}; },
            's' => { if started { buf.push(c); } else {name.push(c)};},
            '>' => { rule.cat = *buf.get(0).unwrap(); buf = Vec::new(); rule.lt = false; },
            '<' => { rule.cat = *buf.get(0).unwrap(); buf = Vec::new(); rule.lt = true; },
            ':' => { rule.val = buf.iter().collect::<String>().parse::<i32>().unwrap(); buf = Vec::new() ;},
            ',' => {
                rule.dest = buf.iter().collect();
                buf = Vec::new();
                wf.push(rule.clone());
                if (rule.cat != '.') {
                    bounds.entry(rule.cat).or_insert(Vec::new()).push(rule.val);
                }
                rule = Rule {cat: '.', lt: false, val: 0, dest: "".to_string()};
            },
            '}' => {
                rule.dest = buf.iter().collect();
                buf = Vec::new();
                wf.push(rule.clone());
                if (rule.cat != '.') {
                    bounds.entry(rule.cat).or_insert(Vec::new()).push(rule.val);
                }
                rule = Rule {cat: '.', lt: false, val: 0, dest: "".to_string()};
            },
            _ => { if started { buf.push(c);} else {name.push(c)}},
        }
    }
    return (name, wf, bounds);
}

pub fn part1(s: String) {
    let mut workflows: HashMap::<String, Vec::<Rule>> = HashMap::new();
    let mut pieces: Vec<HashMap::<char, i32>> = Vec::new();
    let mut emptyline = false;
    for line in s.lines() {
        if line.trim() == "" {
            emptyline = true;
        } else {
            if emptyline {
                pieces.push(parse_piece(line));
            } else {
                let mut name = "";
                let (name, workflow, _) = parse_workflow(line);
                workflows.insert(name.clone(), workflow);
            }
        }
    }


    let mut sum = 0;

    for p in pieces {
        let res = process_workflow(&p, workflows.clone(), "in".to_string());
        if res == "A" {
            sum += piece_value(p);
        }
    }

    println!("{}", sum);
}

#[derive(Debug)]
struct Cube {
    xmin: i32,
    xmax: i32,
    mmin: i32,
    mmax: i32,
    amin: i32,
    amax: i32,
    smin: i32,
    smax: i32,
}

fn get_valid_cubes(c: Cube, workflows: HashMap::<String, Vec::<Rule>>, name: String) -> Vec::<Cube> {

    let mut res: Vec::<Cube> = Vec::new();
    if (name == "R") {
        return res;
    }
    if (name == "A") {
        res.push(c);
        return res;
    }
    let wf = workflows.get(&name).expect("workflow");
    
    for rule in wf {
        if rule.cat == '.' {
            return get_valid_cubes(c, workflows.clone(), rule.dest.clone());
        }

        match rule.cat {
            'x' => {
                if rule.lt {
                    if c.xmax < rule.val {
                        return get_valid_cubes(c, workflows.clone(), rule.dest.clone())
                    } else if c.xmin >= rule.val {
                        continue;
                    }
                } else if !rule.lt {
                    if  c.xmin > rule.val {
                        return get_valid_cubes(c, workflows.clone(), rule.dest.clone())
                    } else if c.xmax <= rule.val {
                        continue;
                    }
                }
                let c1;
                let c2;
                if (rule.lt) {
                    c1 = Cube { xmin: c.xmin, xmax: rule.val-1, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: c.smin, smax:c.smax};
                    c2 = Cube { xmin: rule.val, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: c.smin, smax:c.smax};
                } else {
                    c1 = Cube { xmin: c.xmin, xmax: rule.val, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: c.smin, smax:c.smax};
                    c2 = Cube { xmin: rule.val+1, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: c.smin, smax:c.smax};
                }
                res = get_valid_cubes(c1, workflows.clone(), name.clone());
                res.append(&mut get_valid_cubes(c2, workflows.clone(), name.clone()));
                return res;
            },
            'm' => {
                if rule.lt {
                    if c.mmax < rule.val {
                        return get_valid_cubes(c, workflows.clone(), rule.dest.clone())
                    } else if c.mmin >= rule.val {
                        continue;
                    }
                } else if !rule.lt {
                    if  c.mmin > rule.val {
                        return get_valid_cubes(c, workflows.clone(), rule.dest.clone())
                    } else if c.mmax <= rule.val {
                        continue;
                    }
                }
                let c1;
                let c2;
                if (rule.lt) {
                    c1 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: rule.val-1, amin:c.amin, amax: c.amax, smin: c.smin, smax:c.smax};
                    c2 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: rule.val, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: c.smin, smax:c.smax};
                } else {
                    c1 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: rule.val, amin:c.amin, amax: c.amax, smin: c.smin, smax:c.smax};
                    c2 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: rule.val+1, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: c.smin, smax:c.smax};
                }
                res = get_valid_cubes(c1, workflows.clone(), name.clone());
                res.append(&mut get_valid_cubes(c2, workflows.clone(), name.clone()));
                return res;
            },
            'a' => {
                if rule.lt {
                    if c.amax < rule.val {
                        return get_valid_cubes(c, workflows.clone(), rule.dest.clone())
                    } else if c.amin >= rule.val {
                        continue;
                    }
                } else if !rule.lt {
                    if  c.amin > rule.val {
                        return get_valid_cubes(c, workflows.clone(), rule.dest.clone())
                    } else if c.amax <= rule.val {
                        continue;
                    }
                }
                let c1;
                let c2;
                if (rule.lt) {
                    c1 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: rule.val-1, smin: c.smin, smax:c.smax};
                    c2 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:rule.val, amax: c.amax, smin: c.smin, smax:c.smax};
                } else {
                    c1 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: rule.val, smin: c.smin, smax:c.smax};
                    c2 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:rule.val+1, amax: c.amax, smin: c.smin, smax:c.smax};
                }
                res = get_valid_cubes(c1, workflows.clone(), name.clone());
                res.append(&mut get_valid_cubes(c2, workflows.clone(), name.clone()));
                return res;
            },
            's' => {
                if rule.lt {
                    if c.smax < rule.val {
                        return get_valid_cubes(c, workflows.clone(), rule.dest.clone())
                    } else if c.smin >= rule.val {
                        continue;
                    }
                } else if !rule.lt {
                    if  c.smin > rule.val {
                        return get_valid_cubes(c, workflows.clone(), rule.dest.clone())
                    } else if c.smax <= rule.val {
                        continue;
                    }
                }
                let c1;
                let c2;
                if (rule.lt) {
                    c1 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: c.smin, smax:rule.val-1};
                    c2 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: rule.val, smax:c.smax};
                } else {
                    c1 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: c.smin, smax:rule.val};
                    c2 = Cube { xmin: c.xmin, xmax: c.xmax, mmin: c.mmin, mmax: c.mmax, amin:c.amin, amax: c.amax, smin: rule.val+1, smax:c.smax};
                }
                res = get_valid_cubes(c1, workflows.clone(), name.clone());
                res.append(&mut get_valid_cubes(c2, workflows.clone(), name.clone()));
                return res;
            },
            _ => {return res;}
        };
    }
    return res;
}

pub fn part2(s: String) {
    let mut workflows: HashMap::<String, Vec::<Rule>> = HashMap::new();

    for line in s.lines() {
        if line.trim() == "" {
            break;
        } else {
            let mut name = "";
            let (name, workflow, mut bound) = parse_workflow(line);
            workflows.insert(name.clone(), workflow);
        }
    }

    let cube = Cube{xmin: 1, xmax: 4000, mmin: 1, mmax: 4000, amin: 1, amax: 4000, smin: 1, smax: 4000};
    let res = get_valid_cubes(cube, workflows, "in".to_string());
    let mut sum: u64 = 0;
    for c in res {
        sum += TryInto::<u64>::try_into(c.xmax - c.xmin + 1).unwrap() * 
            TryInto::<u64>::try_into(c.mmax - c.mmin + 1).unwrap() * 
            TryInto::<u64>::try_into(c.amax - c.amin + 1).unwrap() *
            TryInto::<u64>::try_into(c.smax - c.smin + 1).unwrap();
    }
    println!("{}", sum);
}