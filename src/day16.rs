#[path = "utils.rs"]
mod utils;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn get_energy(grid: Vec::<Vec<char>>, energy: &mut HashMap::<(i32, i32), HashSet::<char>>, row: i32, col: i32, dir: char) {
    let mut queue: VecDeque<(i32, i32, char)> = VecDeque::new();
    queue.push_front((row, col, dir));
    while queue.len() > 0 {
        let elem = queue.pop_front().expect("elem");
        if elem.0 < 0 || elem.1 < 0 || elem.0 >= grid.len().try_into().unwrap() || elem.1 >= grid.get(0).expect("row").len().try_into().unwrap() {
            continue;
        }
        if energy.entry((elem.0, elem.1)).or_insert(HashSet::new()).contains(&elem.2) {
            continue;
        } else {
            energy.entry((elem.0, elem.1)).and_modify( |x| {x.insert(elem.2); } );
        }
        match grid.get::<usize>(elem.0.try_into().unwrap()).expect("row").get::<usize>(elem.1.try_into().unwrap()).expect("col") {
            '.' => {
                match elem.2 {
                    'N' => {queue.push_front((elem.0-1, elem.1, 'N'))},
                    'S' => {queue.push_front((elem.0+1, elem.1, 'S'))},
                    'E' => {queue.push_front((elem.0, elem.1+1, 'E'))},
                    'W' => {queue.push_front((elem.0, elem.1-1, 'W'))},
                    _ => {}
                };
            },
            '/' => {
                match elem.2 {
                    'N' => {queue.push_front((elem.0, elem.1+1, 'E'))},
                    'S' => {queue.push_front((elem.0, elem.1-1, 'W'))},
                    'E' => {queue.push_front((elem.0-1, elem.1, 'N'))},
                    'W' => {queue.push_front((elem.0+1, elem.1, 'S'))},
                    _ => {}
                };
            },
            '\\' => {
                match elem.2 {
                    'S' => {queue.push_front((elem.0, elem.1+1, 'E'))},
                    'N' => {queue.push_front((elem.0, elem.1-1, 'W'))},
                    'W' => {queue.push_front((elem.0-1, elem.1, 'N'))},
                    'E' => {queue.push_front((elem.0+1, elem.1, 'S'))},
                    _ => {}
                };
            },
            '|' => {
                match elem.2 {
                    'N' => {queue.push_front((elem.0-1, elem.1, 'N'))},
                    'S' => {queue.push_front((elem.0+1, elem.1, 'S'))},
                    'E' => {queue.push_front((elem.0-1, elem.1, 'N')); queue.push_front((elem.0+1, elem.1, 'S'))},
                    'W' => {queue.push_front((elem.0-1, elem.1, 'N')); queue.push_front((elem.0+1, elem.1, 'S'))},
                    _ => {}
                }
            },
            '-' => {
                match elem.2 {
                    'N' => { queue.push_front((elem.0, elem.1 - 1, 'W')); queue.push_front((elem.0, elem.1 + 1, 'E'));}
                    'S' => { queue.push_front((elem.0, elem.1 - 1, 'W')); queue.push_front((elem.0, elem.1 + 1, 'E'));}
                    'E' => {queue.push_front((elem.0, elem.1+1, 'E'))},
                    'W' => {queue.push_front((elem.0, elem.1-1, 'W'))},
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

pub fn part1(s: String) {
    let grid = utils::get_grid(s);
    let mut energy: HashMap::<(i32, i32), HashSet::<char>> = HashMap::new();
    get_energy(grid, &mut energy, 0, 0, 'E');
    println!("{}", energy.len());
}

pub fn part2(s: String) {
    let grid = utils::get_grid(s);
    let maxrow = grid.len();
    let maxcol = grid.get(0).expect("row").len() - 1;

    let mut maxenergy = 0;
    
    for row in 0..maxrow {
        let mut energy: HashMap::<(i32, i32), HashSet::<char>> = HashMap::new();
        get_energy(grid.clone(), &mut energy, row.try_into().unwrap(), 0, 'E');
        maxenergy = std::cmp::max(maxenergy, energy.len());
        energy = HashMap::new();
        get_energy(grid.clone(), &mut energy, row.try_into().unwrap(), maxcol.try_into().unwrap(), 'W');
        maxenergy = std::cmp::max(maxenergy, energy.len());
    }

    for col in 0..maxcol {
        let mut energy: HashMap::<(i32, i32), HashSet::<char>> = HashMap::new();
        get_energy(grid.clone(), &mut energy, 0, col.try_into().unwrap(), 'S');
        maxenergy = std::cmp::max(maxenergy, energy.len());
        energy = HashMap::new();
        get_energy(grid.clone(), &mut energy, maxrow.try_into().unwrap(), col.try_into().unwrap(), 'N');
        maxenergy = std::cmp::max(maxenergy, energy.len());
    }
    
    println!("{}", maxenergy);
}