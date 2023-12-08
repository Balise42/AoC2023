/*mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;*/
mod day08;

fn main() {
    /*day01::part1(read_test_input(1));
    day01::part1(read_input(1));
    day01::part2(read_test_input(1));
    day01::part2(read_input(1));
    day02::part1(read_test_input(2));
    day02::part1(read_input(2));
    day02::part2(read_test_input(2));
    day02::part2(read_input(2));
    day03::part1(read_test_input(3));
    day03::part1(read_input(3));
    day03::part2(read_test_input(3));
    day03::part2(read_input(3));
    day04::part1(read_test_input(4));
    day04::part1(read_input(4));
    day04::part2(read_test_input(4));
    day04::part2(read_input(4));
    day05::part1(read_test_input(5));
    day05::part1(read_input(5));*
    day05::part2(read_test_input(5));
    day05::part2(read_input(5));
    day06::part1(read_test_input(6));
    day06::part1(read_input(6));
    day06::part2(read_test_input(6));
    day06::part2(read_input(6));*/
    /*day07::part1(read_test_input(7));
    day07::part1(read_input(7));
    day07::part2(read_test_input(7));
    day07::part2(read_input(7));*/
    day08::part1(read_test_input(8));
    day08::part1(read_input(8));
    day08::part2(read_test_input(8));
    day08::part2(read_input(8));
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}

fn read_test_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/test{:0>2}.txt", day)).unwrap()
}