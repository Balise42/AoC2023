pub fn get_grid(s: String) -> Vec::<Vec::<char>> {
    let mut grid: Vec::<Vec::<char>> = Vec::new();
    for line in s.lines() {
        grid.push(line.chars().collect());
    }
    return grid;
}