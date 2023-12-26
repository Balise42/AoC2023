use std::collections::HashMap;

fn make_graph(s: String) -> HashMap<String, Vec::<String>> {
    let mut graph: HashMap::<String, Vec::<String>> = HashMap::new();
    for line in s.lines() {
        let mut toks = line.split(": ");
        let v1 = toks.next().unwrap().to_string();
        for v in toks.next().unwrap().split(" ") {
            graph.entry(v1.clone()).or_insert(Vec::new()).push(v.to_string());
            graph.entry(v.to_string()).or_insert(Vec::new()).push(v1.clone());
        }
    }
    return graph;
}

fn compute_cut(graph: &HashMap::<String, Vec::<String>>, part: &Vec::<String>) -> usize {
    let mut cut = 0;
    for v in part {
        let neighs = graph.get(v).unwrap();
        for n in neighs {
            if !part.contains(n) {
                cut+=1;
            }
        } 
    }
    return cut;
}

fn merge_nodes(graph: &mut HashMap::<String, Vec::<String>>, part: &mut Vec::<String>) {
    let s = part.pop().unwrap();
    let t = part.pop().unwrap();
    let newv = s.clone() + &"-" + &t;
    let mut newedge = Vec::new();
    for e in graph.get(&s).unwrap().clone() {
        if *e != t {
            newedge.push(e.clone());
            let mut merged = Vec::new();
            for k in graph.get(&e).unwrap() {
                if *k == s {
                    merged.push(newv.clone());
                } else {
                    merged.push(k.clone());
                }
            }
            graph.insert(e.clone(), merged);
        }
    }
    for e in graph.get(&t).unwrap().clone() {
        if *e != s {
            newedge.push(e.clone());
            let mut merged = Vec::new();
            for k in graph.get(&e).unwrap() {
                if *k == t {
                    merged.push(newv.clone());
                } else {
                    merged.push(k.clone());
                }
            }
            graph.insert(e.clone(), merged);
        }
    }
    graph.insert(newv, newedge);
    graph.remove(&s);
    graph.remove(&t);
}

fn get_cut(graph: &mut HashMap::<String, Vec::<String>>) -> (Vec::<String>, Vec::<String>) {
    
    for i in 0..graph.len() {
        let mut nodes: Vec::<String> = graph.keys().cloned().collect();

        let mut A: Vec::<String> = Vec::new();
        let mut to_insert = nodes.pop().unwrap();
        let mut cutsizes : HashMap<String, usize> = HashMap::new();
        for n in &nodes {
            cutsizes.insert(n.clone(), 0);
        }
        while nodes.len() > 0 {
            A.push(to_insert.clone());
            for n in graph.get(&to_insert).unwrap() {
                cutsizes.entry(n.clone()).and_modify(|x| *x+=1);
            }
            nodes.sort_by_key(|a| cutsizes.get(a).unwrap());
            if nodes.len() == 1 {
                let cut = compute_cut(&graph, &A);
                if  cut == 3 {
                    return (nodes, A);
                }
                println!("{} {}", i, cut);
            }
            to_insert = nodes.pop().unwrap();
        }
        A.push(to_insert);
        merge_nodes(graph, &mut A);
    }
    
    return (Vec::new(), Vec::new());
}

pub fn part1(s: String) {
    let mut graph: HashMap::<String, Vec::<String>> = make_graph(s);
    println!("{}", graph.len());
    let (a, b) = get_cut(&mut graph);

    let mut numa = 0;
    for v in &a {
        numa += v.split('-').collect::<Vec::<&str>>().len();
    }
    let mut numb = 0;
    for v in &b {
        numb += v.split('-').collect::<Vec::<&str>>().len();
    }
    println!("{} {} {}", numa, numb, numa * numb);
}