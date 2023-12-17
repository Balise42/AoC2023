#[path = "utils.rs"]
mod utils;

use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    row: usize,
    col: usize,
    dir: char,
    steps: usize,
}

fn get_dir(row: usize, col: usize, maxrow: usize, maxcol: usize, dir: char) -> Option<(usize, usize)> {
    match dir {
        'N' => { if row > 0 { Some((row-1, col)) } else { None } },
        'S' => { if row < maxrow {Some((row+1, col))} else {None} },
        'E' => { if col < maxcol {Some((row, col+1))} else {None} },
        'W' => { if col > 0 {Some((row, col-1))} else {None} },
        _ => { None }
    }
}

fn left(dir: char) -> Option<char> {
    match dir {
        'N' => Some('W'),
        'S' => Some('E'),
        'E' => Some('S'),
        'W' => Some('N'),
        _ => None
    }
}

fn right(dir: char) -> Option<char> {
    match dir {
        'N' => Some('E'),
        'S' => Some('W'),
        'E' => Some('N'),
        'W' => Some('S'),
        _ => None
    }
}

fn get_neighbors(n: &Node, maxrow: usize, maxcol: usize, minstep: usize, maxstep: usize) -> Vec::<Node> {
    let mut res: Vec::<Node> = Vec::new();
    if n.steps >= minstep {
        let leftd = left(n.dir);
        match leftd {
            Some(d) => {
                let leftn = get_dir(n.row, n.col, maxrow, maxcol, d);
                match leftn {
                    Some(x) => { res.push( Node {row: x.0, col: x.1, dir: d, steps: 1})},
                    None => {}
                }
            },
            None => {}
        }

        let rightd = right(n.dir);
        match rightd {
            Some(d) => {
                let rightn = get_dir(n.row, n.col, maxrow, maxcol, d);
                match rightn {
                    Some(x) => { res.push( Node {row: x.0, col: x.1, dir: d, steps: 1})},
                    None => {}
                }
            },
            None => {}
        }
    }

    if n.steps < maxstep {
        let nextn = get_dir(n.row, n.col, maxrow, maxcol, n.dir);
        match nextn {
            Some(x) => { res.push( Node {row: x.0, col: x.1, dir: n.dir, steps: n.steps + 1})},
            None => {}
        }
    }
    return res;
}

fn move_crucible(grid: Vec::<Vec::<usize>>, minstep: usize, maxstep: usize) {
    let maxrow = grid.len()-1;
    let maxcol = grid.get(0).unwrap().len() - 1;

    let mut pq : PriorityQueue::<Node, Reverse<usize>> = PriorityQueue::new();
    let mut visited: HashMap::<Node, usize> = HashMap::new();

    let u1 = Node{row: 0, col: 0, dir: 'E', steps: 0};
    visited.insert(u1.clone(), 0);
    pq.push(u1, Reverse(0));
    let u2 = Node{row: 0, col: 0, dir: 'S', steps: 0};
    visited.insert(u2.clone(), 0);
    pq.push(u2, Reverse(0));


    while !pq.is_empty() {
        let u = pq.pop().unwrap().0;

        if u.row == maxrow && u.col == maxcol {
            println!("{:?}", visited.get(&u).unwrap());
            break;
        }
    
        let neighbors = get_neighbors(&u, maxrow, maxcol, minstep, maxstep);
        for v in neighbors {
            let alt = visited.get(&u).expect("u") + grid.get(v.row).expect("row").get(v.col).expect("col");
            if !visited.contains_key(&v) || alt < *visited.get(&v).unwrap() {
                visited.insert(v.clone(), alt);
                pq.push(v, Reverse(alt));
            }
        }
    }
}

pub fn part1(s: String) {
    let grid = utils::get_int_grid(s);
    move_crucible(grid, 0, 3);
}

pub fn part2(s: String) {
    let grid = utils::get_int_grid(s);
    move_crucible(grid, 4, 10);
}