use std::collections::HashMap;

fn getGrid(s: String) -> Vec::<Vec::<char>> {
    let mut grid: Vec::<Vec::<char>> = Vec::new();
    for line in s.lines() {
        grid.push(line.chars().collect());
    }
    return grid;
}

fn getLoad(grid: Vec::<Vec::<char>>) -> usize {
    let mut totLoad: usize = 0;
    let colSize: usize = grid.get(0).expect("row").len();

    for col in 0..colSize {
        for row in 0..grid.len() {
            if *grid.get(row).expect("row").get(col).expect("col") == 'O' {
                totLoad += colSize - row;
            }
        }
    }
    return totLoad;
}

fn create_empty_grid(grid: &Vec::<Vec::<char>>) -> Vec::<Vec::<char>> {
    let mut res: Vec::<Vec::<char>> = Vec::new();
    for row in grid {
        res.push(Vec::new());
        for col in row {
            if *col == '#' {
                res.last_mut().expect("last").push('#');
            } else {
                res.last_mut().expect("last").push('.');
            }
        }
    }
    return res;
}

fn tilt_north(grid: Vec::<Vec::<char>>) -> Vec::<Vec::<char>> {
    let mut res = create_empty_grid(&grid);
    for col in 0..grid.get(0).expect("row").len() {
        let mut pempt = 0;
        while pempt < grid.len() && *grid.get(pempt).expect("row").get(col).expect("col") != '.' {
            if *grid.get(pempt).expect("row").get(col).expect("col") == 'O' {
                res[pempt][col] = 'O';
            }
            pempt += 1;
        }
        let mut moving = pempt + 1;
        while moving < grid.len() {
            if *grid.get(moving).expect("row").get(col).expect("col") == 'O' {
                res[pempt][col] = 'O';
                pempt+=1;
            } else if *grid.get(moving).expect("row").get(col).expect("col") == '#' {
                pempt = moving + 1;
                if ( pempt >= grid.len()) {
                    break;
                }
                while pempt < grid.len() && *grid.get(pempt).expect("row").get(col).expect("col") != '.' {
                    if *grid.get(pempt).expect("row").get(col).expect("col") == 'O' {
                        res[pempt][col] = 'O';
                    }
                    pempt += 1;
                }
                moving = pempt;
            }
            moving +=1;
        }
    }
    return res;
}

fn rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();
    for i in 0..grid.get(0).expect("row").len() {
        res.push(Vec::new());
    }

    let n = grid.get(0).expect("row").len();
    for row in 0..grid.len() {
        for col in 0..grid.get(0).expect("row").len() {
            res[n-col-1].push(*grid.get(row).expect("row").get(col).expect("col"));
        }
    }
    return res;
}

fn tilt_south(grid: Vec::<Vec::<char>>) -> Vec::<Vec::<char>> {
    let rotated = rotate(rotate(grid));
    let res = tilt_north(rotated);
    return rotate(rotate(res));
}

fn tilt_west(grid: Vec::<Vec::<char>>) -> Vec::<Vec::<char>> {
    let rotated = rotate(rotate(rotate(grid)));
    let res = tilt_north(rotated);
    return rotate(res);
}

fn tilt_east(grid: Vec::<Vec::<char>>) -> Vec::<Vec::<char>> {
    let rotated = rotate(grid);
    let res = tilt_north(rotated);
    return rotate(rotate(rotate(res)));
}

fn cycle(grid: Vec::<Vec::<char>>) -> Vec::<Vec::<char>> {
    let new_grid = tilt_east(tilt_south(tilt_west(tilt_north(grid))));
    return new_grid;
}

pub fn part1(s: String) {
    let grid = getGrid(s);
    let load = getLoad(tilt_north(grid));
    println!("{:?}", load);
}

pub fn part2(s: String) {
    let mut seen: HashMap::<Vec::<Vec::<char>>, i32> = HashMap::new();
    let grid = getGrid(s);
    let mut newgrid = grid.clone();
    let mut length = 0;
    let mut first = 0;
    for i in 0..100 {
        newgrid = cycle(newgrid.clone());
        if seen.contains_key(&newgrid) {
            length = i - seen.get(&newgrid).expect("seen");
            first = i-length;
            break;
        }
        seen.insert(newgrid.clone(), i);
    }
    let index = (1000000000 - 1 - first) % length + first;
    println!("{} {} {}", first, length, index);
    for (k, v) in seen {
        if v == index {
            println!("{}", getLoad(k));
        }
    }
}