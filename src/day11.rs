use std::collections::HashSet;
use core::ops::RangeInclusive;

fn get_grid(s: String) -> (Vec::<(usize,usize)>, HashSet::<usize>, HashSet::<usize>) {
    let mut grid: Vec::<Vec::<char>> = Vec::new();
    for line in s.lines() {
        grid.push(line.chars().collect());
    }
    let mut empty_rows: HashSet<usize> = HashSet::from_iter(0..grid.len());
    let mut empty_cols: HashSet<usize> = HashSet::from_iter(0..grid.get(0).expect("row").len());
    let mut coords : Vec::<(usize, usize)> = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid.get(0).expect("row").len() {
            if *grid.get(row).expect("row").get(col).expect("col") == '#' {
                empty_rows.remove(&row);
                empty_cols.remove(&col);
                coords.push((row, col));
            }
        }
    }
    return (coords, empty_rows, empty_cols)
}

fn get_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        return a..=b;
    }
    return b..=a;
}

pub fn part1(s: String) {
    let mut sum: i32 = 0;
    let (coords, rows, cols) = get_grid(s);
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let a1 = coords.get(i).expect("first").0 as i32;
            let b1 = coords.get(j).expect("second").0 as i32;
            let a2 = coords.get(i).expect("first").1 as i32;
            let b2 = coords.get(j).expect("second").1 as i32;
            sum += (a1 - b1).abs();
            sum += (a2 - b2).abs();
            let addRows: i32 = rows.intersection(&HashSet::from_iter(get_range(a1.try_into().unwrap(), b1.try_into().unwrap()))).count().try_into().unwrap();
            sum += addRows;
            let addCols: i32 = cols.intersection(&HashSet::from_iter(get_range(a2.try_into().unwrap(), b2.try_into().unwrap()))).count().try_into().unwrap();
            sum += addCols;
        }
    }
    println!("{}", sum)
}


pub fn part2(s: String) {
    let mut sum: i64 = 0;
    let (coords, rows, cols) = get_grid(s);
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let a1 = coords.get(i).expect("first").0 as i64;
            let b1 = coords.get(j).expect("second").0 as i64;
            let a2 = coords.get(i).expect("first").1 as i64;
            let b2 = coords.get(j).expect("second").1 as i64;
            sum += (a1 - b1).abs();
            sum += (a2 - b2).abs();
            let addRows: i64 = rows.intersection(&HashSet::from_iter(get_range(a1.try_into().unwrap(), b1.try_into().unwrap()))).count().try_into().unwrap();
            sum += 999999 * addRows;
            let addCols: i64 = cols.intersection(&HashSet::from_iter(get_range(a2.try_into().unwrap(), b2.try_into().unwrap()))).count().try_into().unwrap();
            sum += 999999 * addCols;
        }
    }
    println!("{}", sum)
}