use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_circuit_line(line: &str) -> (&str, (char, Vec::<&str>)) {
    let mut toks = line.split(" -> ");
    let left = toks.next().unwrap();
    let dests = toks.next().unwrap().split(", ").collect();
    if left == "broadcaster" {
        return("broadcaster", ('B', dests));
    } else {
        let (gtype, name) = left.split_at(1);
        if gtype == "%" {
            return (name, ('F', dests));
        } else {
            return(name, ('C', dests));
        }
    }
}

fn run_circuit<'a>(circuit: &HashMap::<&str, (char, Vec::<&'a str>)>, ff_state: &mut HashMap::<&'a str, bool>, con_state: &mut HashMap::<&'a str, HashMap::<&'a str, bool>>, search: &str) -> (u64, u64, &'a str) {
    let mut instr: VecDeque::<(&str, bool, &str)> = VecDeque::new();
    let mut low = 1;
    let mut high = 0;
    let mut found = "";
    for dest in &circuit.get("broadcaster").unwrap().1 {
        instr.push_back((dest, false, "broadcaster"));
        low += 1;
    }
    while instr.len() > 0 {
        let inst = instr.pop_front().unwrap();
        match circuit.get(inst.0) {
            Some(x) => { let pos = x; },
            None => { continue },
        };
        let pos = circuit.get(inst.0).unwrap();
        if pos.0 == 'F' {
            if inst.1 {
                continue;
            }
            let state = !ff_state.get(inst.0).unwrap();
            ff_state.insert(inst.0, state);
            for dest in &pos.1 {
                instr.push_back((dest, state, inst.0));
                if state {
                    high += 1;
                } else {
                    low +=1;
                }
            }
        } else if pos.0 == 'C' {
            con_state.entry(inst.0).or_insert(HashMap::new()).insert(inst.2, inst.1);
            let mut res = true;
            for (_, b) in con_state.get(inst.0).unwrap().iter() {
                res = res && *b;
            }
            for dest in &pos.1 {
                instr.push_back((dest, !res, inst.0));
                if res {
                    low +=1;
                } else {
                    high +=1;
                }
            }
            if ( inst.0 == search ) && !res {
                found = inst.0;
            }
        }
    }
    return (low, high, found);
}

pub fn part1(s: String) {
    let (circuit, mut ff_state, mut con_state) = reinit_circuit(&s);

    let mut suml = 0;
    let mut sumh = 0;
    for i in 0..1000 {
        let (low, high, _) = run_circuit(&circuit, &mut ff_state, &mut con_state, "");
        suml += low;
        sumh += high;
    }
    println!("{}", suml * sumh);
}

pub fn reinit_circuit<'a>(s: &'a String)-> (HashMap::<&'a str, (char, Vec::<&'a str>)>, HashMap::<&'a str, bool>, HashMap::<&'a str, HashMap::<&'a str, bool>>) {
    let mut circuit: HashMap::<&str, (char, Vec::<&str>)> = HashMap::new();
    for line in s.lines() {
        let (name, val) = parse_circuit_line(line);
        circuit.insert(name, val);
    }
    let mut ff_state: HashMap::<&str, bool> = HashMap::new();
    let mut con_state: HashMap::<&str, HashMap::<&str, bool>> = HashMap::new();

    for (key, val) in circuit.iter() {
        if val.0 == 'F' {
            ff_state.insert(key, false);
        }
        if val.0 == 'C' {
            con_state.insert(key, HashMap::new());
        }
    }

    for (key, val) in circuit.iter() {
        for dest in &val.1 {
            if con_state.contains_key(dest) {
                con_state.entry(dest).and_modify(|s| { s.insert(key, false);});
            }
        }
    }
    return (circuit, ff_state, con_state);
}

pub fn part2(s: String) {
    let mut res: u64 = 1;

    for st in ["cm", "sz", "xf", "gc"] {
        let (circuit, mut ff_state, mut con_state) = reinit_circuit(&s);
        let mut it = 0;
        let mut found = "";
        while found != st {
            (_, _, found) = run_circuit(&circuit, &mut ff_state, &mut con_state, st);
            it += 1;
        }
        println!("{} {}", st, it);
        res *= it;
    }
    println!("{}", res);
}  