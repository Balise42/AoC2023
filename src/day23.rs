use std::collections::HashMap;
use memoize::memoize;
#[path = "utils.rs"]
mod utils;

fn get_neighbors(grid: &Vec::<Vec::<char>>, row: usize, col: usize, path: &Vec::<(usize, usize)>) -> Vec::<(usize, usize)> {
    let maxrow = grid.len();
    let maxcol = grid.get(0).unwrap().len();

    let mut res: Vec::<(usize, usize)> = Vec::new();
    if row > 0 {
        let neigh = *grid.get(row-1).unwrap().get(col).unwrap();
        if (neigh == '.' || neigh == '^') && !path.contains(&(row-1, col)) {
            res.push((row-1, col));
        }
    }
    if col > 0 {
        let neigh = *grid.get(row).unwrap().get(col-1).unwrap();
        if (neigh == '.' || neigh == '<') && !path.contains(&(row, col-1)) {
            res.push((row, col-1));
        }
    }
    if row < maxrow - 1 {
        let neigh = *grid.get(row+1).unwrap().get(col).unwrap();
        if (neigh == '.' || neigh == 'v') && !path.contains(&(row+1, col)) {
            res.push((row+1, col));
        }
    }
    if col < maxcol - 1 {
        let neigh = *grid.get(row).unwrap().get(col+1).unwrap();
        if (neigh == '.' || neigh == '>') && !path.contains(&(row, col+1)) {
            res.push((row, col+1));
        }
    }
    return res;
}

fn get_max_path(grid: &Vec::<Vec::<char>>, path: Vec::<(usize, usize)>, startr: usize, startc: usize, endr: usize, endc: usize) -> Option::<i32> {
    let last = (startr, startc);
    if last.0 == endr && last.1 == endc {
        return Some(0);
    }
    let mut neighs = get_neighbors(&grid, last.0, last.1, &path);
    if neighs.len() == 0 {
        return None;
    }
    let mut length = 0;
    let mut newpath = path.clone();
    if neighs.len() == 1 {
        while neighs.len() == 1 {
            let neigh = *neighs.get(0).unwrap();
            newpath.push(neigh);
            length += 1;

            if (neigh.0 == endr && neigh.1 == endc) {
                return Some(length);
            }
            neighs = get_neighbors(&grid, neigh.0, neigh.1, &newpath);
        }
    }

    let mut res = -1;
    for neigh in neighs {
        newpath.push(neigh);
        let next = get_max_path(grid, newpath.clone(), neigh.0, neigh.1, endr, endc);
        match next {
            Some(x) => { res = std::cmp::max(res, x); }
            None => {}
        }
    }
    if res != -1 {
        return Some(res+length+1);
    }
    return None;

}

pub fn part1(s: String) {
    let grid = utils::get_grid(s);
    let startr = 0;
    let endr = grid.len()-1;
    let mut startc = 0;
    let mut endc = 0;
    for i in 0..grid.get(startr).unwrap().len() {
        if *grid.get(startr).unwrap().get(i).unwrap() == '.' {
            startc = i;
            break;
        }
    }
    for i in 0..grid.get(endr).unwrap().len() {
        if *grid.get(endr).unwrap().get(i).unwrap() == '.' {
            endc = i;
            break;
        }
    }
    let mut path: Vec::<(usize, usize)> = Vec::new();
    path.push((startr, startc));
    println!("{:?}", get_max_path(&grid, path, startr, startc, endr, endc));
}

fn get_max_path_graph(graph: &HashMap::<(usize, usize), Vec::<((usize, usize), usize)>>, path: Vec::<(usize, usize)>, startr: usize, startc: usize, endr: usize, endc: usize) -> Option::<i32> {
    if startr == endr && startc == endc {
        return Some(0);
    }
    let node = (startr, startc);
    let neighs = graph.get(&node).unwrap_or(&Vec::<((usize, usize),usize)>::new()).clone();
    let mut res = -1;
    for neigh in neighs {
        if path.contains(&neigh.0) {
            continue;
        }
        let mut newpath = path.clone();
        newpath.push(neigh.0);
        let next = get_max_path_graph(graph, newpath, neigh.0.0, neigh.0.1, endr, endc);
        match next {
            Some(x) => {res = std::cmp::max(res, x + neigh.1 as i32)},
            None => {}
        };
    }

    if res > -1 {
        return Some(res);
    }
    return None;
}

pub fn part2(s: String) {
    let mut grid = utils::get_grid(s);
    
    for i in 0..grid.len() {
        for j in 0..grid.get(0).unwrap().len() {
            if *grid.get(i).unwrap().get(j).unwrap() != '#' {
                grid[i][j] = '.';
            }
        }
    }

    let startr = 0;
    let endr = grid.len()-1;
    let mut startc = 0;
    let mut endc = 0;
    for i in 0..grid.get(startr).unwrap().len() {
        if *grid.get(startr).unwrap().get(i).unwrap() == '.' {
            startc = i;
            break;
        }
    }
    for i in 0..grid.get(endr).unwrap().len() {
        if *grid.get(endr).unwrap().get(i).unwrap() == '.' {
            endc = i;
            break;
        }
    }
    let mut path: Vec::<(usize, usize)> = Vec::new();
    path.push((startr, startc));
    let graph = contract_graph(&grid, startr, startc, endr, endc);
    println!("{:?}", get_max_path_graph(&graph, path, startr, startc, endr, endc));
}

fn contract_graph(grid: &Vec::<Vec::<char>>, startr: usize, startc: usize, endr: usize, endc: usize) -> HashMap::<(usize, usize), Vec::<((usize, usize), usize)>> {
    let mut graph: HashMap::<(usize, usize), Vec::<((usize, usize), usize)>> = HashMap::new();
    let mut node_a = (startr, startc);
    let mut node = (startr+1, startc);
    let mut prev = (startr, startc);
    let mut neighs = get_neighbors(&grid, startr+1, startc, &Vec::from([prev]));
    let mut i: usize = 1;
    while neighs.len() == 1 {
        prev = node.clone();
        node = *neighs.get(0).unwrap();
        neighs = get_neighbors(&grid, node.0, node.1, &Vec::from([prev]));
        i +=1;
    }
    graph.insert(node_a, neighs.iter().map({|x| (*x, i+1)}).collect());
    for n in &neighs {
        contract_edge_from(*n, grid, &mut graph, &mut Vec::from([node_a, node]), node, endr, endc);
    }
    println!("{:?}",neighs);
    for (k, v) in graph.iter() {
        println!("{:?} {:?}", k, v);
    }
    return graph;
}

fn contract_edge_from(start: (usize, usize), grid: &Vec::<Vec::<char>>, graph: &mut HashMap::<(usize, usize), Vec::<((usize, usize), usize)>>, path: &mut Vec::<(usize, usize)>, last: (usize, usize), endr: usize, endc: usize) {
    //println!("{:?}", graph);
    let mut prev = last;
    let mut node = start;
    path.push(prev);
    let mut neighs = get_neighbors(&grid, node.0, node.1, path);
    let mut i: usize = 0;
    while neighs.len() == 1 {
        prev = node.clone();
        node = *neighs.get(0).unwrap();
        path.push(prev);
        neighs = get_neighbors(&grid, node.0, node.1, path);
        i +=1;
    }

    if(node == (endr, endc)) {
        graph.insert(start, Vec::from([(node, i)]));
    }

    if neighs.len() >=2 {
        graph.insert(start, neighs.iter().map({|x| (*x, i)}).collect());
        for n in &neighs {
            contract_edge_from(*n, grid, graph, path, node, endr, endc);
        }
    }
}
