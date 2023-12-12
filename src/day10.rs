fn get_grid(s: String) -> Vec::<Vec::<char>> {
    let mut res: Vec::<Vec::<char>> = Vec::new();
    for line in s.lines() {
        res.push(line.chars().collect());
    }
    res
}

fn get_loop_far(mut grid: Vec::<Vec::<char>>, row: usize, col: usize, dir: char) -> Option::<(i32, Vec::<Vec::<char>>, char)> {
    let mut size: i32 = 0;
    let mut posr = row;
    let mut posc = col;
    let mut d = dir;

    loop {
        match d {
            'N' => { 
                if posr == 0 {
                    return None;
                }
                posr -= 1;
                let new = grid.get(posr).expect("north").get(posc).expect("same");
                match new {
                    '-' => { return None; },
                    'L' => { return None; },
                    'J' => { return None; },
                    '.' => { return None; },
                    '7' => { d = 'W'; },
                    'F' => { d = 'E'; },
                    'S' => { return Some((size, grid, d)); },
                    '|' => {},
                    _ => { return None; }
                }
            },
            'S' => {
                posr += 1;
                if posr >= grid.len() {
                    return None;
                }
                let new = grid.get(posr).expect("south").get(posc).expect("same");
                match new {
                    '-' => { return None; },
                    'F' => { return None; },
                    '7' => { return None; },
                    '.' => { return None; },
                    'J' => { d = 'W'; },
                    'L' => { d = 'E'; },
                    '|' => {},
                    'S' => { return Some((size, grid, d)); },
                    _ => { return None; }
                }
            },
            'W' => { 
                if posc == 0 {
                    return None;
                }
                posc -= 1;
                let new = grid.get(posr).expect("same").get(posc).expect("west");
                match new {
                    '|' => { return None; },
                    '7' => { return None; },
                    'J' => { return None; },
                    '.' => { return None; },
                    'L' => { d = 'N'; },
                    'F' => { d = 'S'; },
                    '-' => {},
                    'S' => { return Some((size, grid, d)); },
                    _ => { return None; }
                }
            },
            'E' => { 
                posc += 1;
                if posc >= grid.get(posr).expect("row").len() {
                    return None;
                }
                let new = grid.get(posr).expect("same").get(posc).expect("east");
                match new {
                    '|' => { return None; },
                    'L' => { return None; },
                    'F' => { return None; },
                    '.' => { return None; },
                    '7' => { d = 'S'; },
                    'J' => { d = 'N'; },
                    '-' => {},
                    'S' => { return Some((size, grid, d)); },
                    _ => { return None; }
                }
            },
            _ => {return None;}
        }
        grid[posr][posc] = match grid.get(posr).expect("row").get(posc).expect("col") {
            '|' => '!',
            '-' => '_',
            'F' => 'P',
            '7' => 'Z',
            'L' => 'E',
            'J' => 'G',
            'S' => 'S',
            _ => '.'
        };
        size += 1;
    }
}

fn get_inside_size(grid: Vec::<Vec::<char>>) -> i32 {
    let mut num: i32 = 0;
    for row in 0..grid.len() {
        let mut inside = false;
        for c in grid.get(row).expect("row") {
            match c {
                '!' => {
                    inside = !inside;
                }
                '_' => {},
                'P' => {
                    inside = !inside;
                },
                'Z' => {
                    inside = !inside;
                },
                'E' => {},
                'G' => {},
                _ => {
                    if inside {
                        num +=1;
                    }
                },
            }
        }
    }
    return num;
}

pub fn part1(s: String) {
    let grid = get_grid(s);
    for row in 0..grid.len() {
        for col in 0..grid.get(0).expect("row").len() {
            if *grid.get(row).expect("row").get(col).expect("col") == 'S' {
                for dir in ['N', 'S', 'W', 'E'] {
                    match get_loop_far(grid.clone(), row, col, dir) {
                        Some((x, _grid, _newdir)) => { println!("{}", (x+1)/2); break; },
                        None => (),
                    }
                }
                break;
            }
        }
    }
}

pub fn part2(s: String) {
    let mut grid = get_grid(s);
    for row in 0..grid.len() {
        for col in 0..grid.get(0).expect("row").len() {
            if *grid.get(row).expect("row").get(col).expect("col") == 'S' {
                for dir in ['N', 'S', 'W', 'E'] {
                    match get_loop_far(grid.clone(), row, col, dir) {
                        Some((x, mut newgrid, newdir)) => {
                            newgrid[row][col] = match (dir, newdir) {
                                ('N', 'N') => '!',
                                ('N', 'E') => 'G',
                                ('N', 'W') => 'E',
                                ('S', 'S') => '!',
                                ('S', 'W') => 'P',
                                ('S', 'E') => 'Z',
                                ('E', 'S') => 'P',
                                ('E', 'E') => '_',
                                ('E', 'N') => 'E',
                                ('W', 'S') => 'Z',
                                ('W', 'W') => '_',
                                ('W', 'N') => 'G',
                                _ => 'S',
                            };
                            
                            println!("{:?}", get_inside_size(newgrid));
                            break;
                        },
                        None => (),
                    }
                }
                break;
            }
        }
    }
}