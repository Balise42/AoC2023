
fn test_grid(grid: Vec<Vec<char>>, row: usize, allow: bool) -> Option::<i32> {
    let mut sum = 0;
    let mut leeway = allow;
    for i in 1..=row {
        if row + i + 1 >= grid.len() {
            if ( leeway ) {
                return None;
            }
            return Some((row+1).try_into().unwrap());
        }
        let lev = levenshtein(grid.get(row + i + 1).expect("i+1"), grid.get(row-i).expect("-i"));
        if lev > 1 || (!leeway && lev > 0) {
            return None;
        }
        if lev == 1 {
            leeway = false;
        }
    }
    if leeway {
        return None;
    }
    return Some(TryInto::<i32>::try_into(row+1).unwrap());
}

fn flip_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();
    for i in 0..grid.get(0).expect("row").len() {
        res.push(Vec::new());
    }

    for row in 0..grid.len() {
        for col in 0..grid.get(0).expect("row").len() {
            res[col].push(*grid.get(row).expect("row").get(col).expect("col"));
        }
    }
    return res;
}

fn levenshtein(a: &Vec<char>, b: &Vec<char>) -> i32 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += if a.get(i).expect("a") != b.get(i).expect("b")  { 1 } else { 0 };
    }
    return sum;
}

fn mirrors(grid : Vec<Vec<char>>, p1: bool) -> Option::<i32> {
    let mut res = 0;
    for i in (0..grid.len()-1) {
        let lev = levenshtein(grid.get(i).expect("i"), grid.get(i+1).expect("i+1"));
        if lev == 0 || (!p1 && lev == 1)  {
            match test_grid(grid.clone(), i, lev == 0 && !p1 ) {
                Some(x) => {return Some(100 * (x)); }
                None => {} 
            }
        }
    }
    let gridhor = flip_grid(grid);
    for i in (0..gridhor.len()-1) {
        let lev = levenshtein(gridhor.get(i).expect("i"), gridhor.get(i+1).expect("i+1"));
        if lev == 0 || (!p1 && lev == 1)  {
            match test_grid(gridhor.clone(), i, lev == 0 && !p1) {
                Some(x) => { return Some(x); }
                None => {} 
            }
        }
    }
    return None;
}

pub fn part1(s: String) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in s.lines() {
        if line.trim() == "" {
            sum += mirrors(grid.clone(), true).expect("mirror");
            grid = Vec::new();
        } else {
            grid.push(line.chars().collect());
        }
    }
    sum += mirrors(grid, true).expect("mirror");
    println!("{}", sum);
}

pub fn part2(s: String) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in s.lines() {
        if line.trim() == "" {
            sum += mirrors(grid.clone(), false).expect("mirror");
            grid = Vec::new();
        } else {
            grid.push(line.chars().collect());
        }
    }
    sum += mirrors(grid, false).expect("mirror");
    println!("{}", sum);
}