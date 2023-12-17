pub fn get_grid(s: String) -> Vec::<Vec::<char>> {
    let mut grid: Vec::<Vec::<char>> = Vec::new();
    for line in s.lines() {
        grid.push(line.chars().collect());
    }
    return grid;
}

pub fn get_int_grid(s: String) -> Vec::<Vec::<usize>> {
    let mut grid: Vec::<Vec::<usize>> = Vec::new();
    for line in s.lines() {
        grid.push(line.chars()
        .map(|x| {x.to_digit(10).unwrap().try_into().unwrap()})
        .collect());
    }
    return grid;
}