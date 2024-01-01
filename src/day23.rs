use std::collections::{HashMap, HashSet};
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
    // 5433 too low
    if startr == endr && startc == endc {
        let mut len = 0;
        for i in 0..path.len()-1 {
            let edges = graph.get(path.get(i).unwrap()).unwrap();
            for edge in edges {
                if edge.0 == *path.get(i+1).unwrap() {
                    len += edge.1;
                }
            }
        }
        println!("{}", len);
        return Some(len as i32);
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
            Some(x) => {res = std::cmp::max(res, x);},
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
    let mut intpoints: HashSet::<(usize, usize)> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid.get(0).unwrap().len() {
            if *grid.get(i).unwrap().get(j).unwrap() == '.' && get_neighbors(grid, i, j, &Vec::new()).len() > 2 {
                intpoints.insert((i, j));
            }
        }
    }
    intpoints.insert((startr, startc));
    intpoints.insert((endr, endc));

    for i in &intpoints {
        let mut path: Vec::<(usize, usize)> = Vec::new();
        path.push(*i);
        graph.insert(*i, Vec::new());
        for neigh in get_neighbors(grid, i.0, i.1, &path) {
            let mut newpath= path.clone();
            let mut n = neigh;
            let mut len = 1;
            newpath.push(neigh);
            while !intpoints.contains(&n) {
                n = *get_neighbors(grid, n.0, n.1, &newpath).get(0).unwrap();
                newpath.push(n);
                len += 1;
            }
            graph.get_mut(&i).unwrap().push((n, len));
        }
    }
    return graph;
}