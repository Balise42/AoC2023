use std::collections::HashSet;

#[path = "utils.rs"]
mod utils;

fn get_steps(grid: &Vec::<Vec::<char>>, last_steps:  &HashSet<(usize, usize)>, num: i32) -> usize {
    if num == 0 {
        return last_steps.len();
    }

    let mut new_steps: HashSet<(usize, usize)> = HashSet::new();
    for s in last_steps {
        if (s.0 > 0) && *grid.get(s.0-1).unwrap().get(s.1).unwrap() != '#' {
            new_steps.insert((s.0-1, s.1));
        }
        if (s.0 < grid.len()-1) && *grid.get(s.0+1).unwrap().get(s.1).unwrap() != '#' {
            new_steps.insert((s.0+1, s.1));
        }
        if (s.1 > 0) && *grid.get(s.0).unwrap().get(s.1-1).unwrap() != '#' {
            new_steps.insert((s.0, s.1-1));
        }
        if (s.1 < grid.get(0).unwrap().len() - 1) && *grid.get(s.0).unwrap().get(s.1+1).unwrap() != '#' {
            new_steps.insert((s.0, s.1+1));
        }
    }
    return get_steps(grid, &new_steps, num-1);
}

fn modulo( num: i32, m: usize ) -> usize {
    let mut res = num % (m as i32);
    if res < 0 {
        res += (m as i32);
    }
    return res as usize;
}

fn get_infinite_steps(grid: &Vec::<Vec::<char>>, last_steps: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {

    let mut new_steps: HashSet<(i32, i32)> = HashSet::new();
    let height = grid.len();
    let width = grid.get(0).unwrap().len();
    for s in last_steps {
        let s0 = modulo(s.0, height);
        let s1 = modulo(s.1, width);
        let s0m = modulo(s.0-1, height);
        let s0p = modulo(s.0 + 1, height);
        let s1m = modulo(s.1-1, width);
        let s1p = modulo(s.1+1, width);


        if *grid.get(s0m).unwrap().get(s1).unwrap() != '#' {
            new_steps.insert((s.0-1, s.1));
        }
        if *grid.get(s0p).unwrap().get(s1).unwrap() != '#' {
            new_steps.insert((s.0+1, s.1));
        }
        if *grid.get(s0).unwrap().get(s1m).unwrap() != '#' {
            new_steps.insert((s.0, s.1-1));
        }
        if *grid.get(s0).unwrap().get(s1p).unwrap() != '#' {
            new_steps.insert((s.0, s.1+1));
        }
    }
    return new_steps;
}

pub fn part1(s: String, num: i32) {
    let grid = utils::get_grid(s);
    let mut last_steps: HashSet<(usize, usize)> = HashSet::new();
    for row in 0..grid.len() {
        for col in 0..grid.get(0).unwrap().len() {
            if *grid.get(row).unwrap().get(col).unwrap() == 'S' {
                last_steps.insert((row, col));
                break;
            }
        }
    }
    println!("{}", get_steps(&grid, &last_steps, num));
}

pub fn part2(s: String, num: i32) {
    let grid = utils::get_grid(s);
    let mut last_steps: HashSet<(i32, i32)> = HashSet::new();
    for row in 0..grid.len() {
        for col in 0..grid.get(0).unwrap().len() {
            if *grid.get(row).unwrap().get(col).unwrap() == 'S' {
                last_steps.insert((row.try_into().unwrap(), col.try_into().unwrap()));
                break;
            }
        }
    }
    let mut prev = 0;
    let mut prevp = 0;
    for i in 1..=num {
        last_steps = get_infinite_steps(&grid, &last_steps);
        let numsteps = last_steps.len();
        if (i % 131 == 65) {
            println!("{} {} {} {}", i, numsteps, numsteps-prev, (numsteps as i32) - (prev as i32)- (prevp as i32));
            prevp = numsteps - prev;
            prev = last_steps.len();
        }
    }
    println!("{}", last_steps.len());
}