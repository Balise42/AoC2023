mod day01;
mod day02;

fn main() {
    /*day01::part1(read_test_input(1));
    day01::part1(read_input(1));
    day01::part2(read_test_input(1));
    day01::part2(read_input(1));*/
    day02::part1(read_test_input(2));
    day02::part1(read_input(2));
    day02::part2(read_test_input(2));
    day02::part2(read_input(2));
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}

fn read_test_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/test{:0>2}.txt", day)).unwrap()
}