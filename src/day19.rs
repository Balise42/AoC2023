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

/*pub fn part2(s: String) {
    let mut workflows: HashMap::<String, Vec::<Rule>> = HashMap::new();
    let mut bounds: HashMap::<char, Vec::<i32>> = HashMap::new();

    bounds.insert('x', Vec::new());
    bounds.insert('m', Vec::new());
    bounds.insert('a', Vec::new());
    bounds.insert('s', Vec::new());

    for line in s.lines() {
        if line.trim() == "" {
            break;
        } else {
            let mut name = "";
            let (name, workflow, mut bound) = parse_workflow(line);
            workflows.insert(name.clone(), workflow);
            for (key, value) in bound.iter_mut() {
                bounds.entry(*key).and_modify(|x| {x.append(value)});
            }
        }
    }

    for bound in bounds.values_mut() {
        for i in bound.clone() {
            bound.push(i+1);
        }
        bound.push(1);
        bound.sort();
        bound.push(4001);
    }

    let mut res: i64 = 0;

    println!("{}", bounds.get(&'x').unwrap().len() * bounds.get(&'m').unwrap().len() * bounds.get(&'m').unwrap().len() * bounds.get(&'s').unwrap().len() );

    for i in 0..bounds.get(&'x').unwrap().len() {
        for j in 0..bounds.get(&'m').unwrap().len() {
            for k in 0..bounds.get(&'a').unwrap().len() {
                for l in 0..bounds.get(&'s').unwrap().len() {
                    let mut piece: HashMap::<char, i32> = HashMap::new();
                    piece.insert('x', *bounds.get(&'x').unwrap().get(i).unwrap());
                    piece.insert('m', *bounds.get(&'m').unwrap().get(j).unwrap());
                    piece.insert('a', *bounds.get(&'a').unwrap().get(k).unwrap());
                    piece.insert('s', *bounds.get(&'s').unwrap().get(l).unwrap());
                    let w = process_workflow(&piece, workflows.clone(), "in".to_string());
                    if w == "A" {
                        let x_width: i64 = (*bounds.get(&'x').unwrap().get(i+1).unwrap() - *bounds.get(&'x').unwrap().get(i).unwrap()).try_into().unwrap();
                        let m_width: i64 = (*bounds.get(&'m').unwrap().get(j+1).unwrap() - *bounds.get(&'m').unwrap().get(j).unwrap()).try_into().unwrap();
                        let a_width: i64 = (*bounds.get(&'a').unwrap().get(k+1).unwrap() - *bounds.get(&'a').unwrap().get(k).unwrap()).try_into().unwrap();
                        let s_width: i64 = (*bounds.get(&'s').unwrap().get(l+1).unwrap() - *bounds.get(&'s').unwrap().get(l).unwrap()).try_into().unwrap();
                        res += x_width * m_width * a_width * s_width;
                    }
                }
            }
        }
    }

    println!("{}", res);
}*/


struct Cube {
    xmin: i32,
    xmax: i32,
    mmin: i32,
    mmax: i32;
    amin: i32,
    amax: i32,
    smin: i32,
    smax: i32,
}

fn get_valid_cubes(c: Cube) {
    ... split the atom.
}

pub fn part2(s: String) {
    let mut workflows: HashMap::<String, Vec::<Rule>> = HashMap::new();

    for line in s.lines() {
        if line.trim() == "" {
            break;
        } else {
            let mut name = "";
            let (name, workflow, mut bound) = parse_workflow(line);
        }
    }

    let cube = Cube{xmin: 1, xmax: 4000, mmin: 1, mmax: 4000, amin: 1, amax: 4000, smin: 1, smax: 4000};
    res = get_valid_cubes(cube);
    let mut sum = 0;
    for c in res {
        sum += (c.xmax - c.xmin + 1) * (c.mmax - c.mmin + 1) * (c.amax - c.amin + 1) + (c.smax - c.smin + 1);
    }
    println!("{}", sum);
}