/*mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;*/
mod day18;

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
    day06::part2(read_input(6));
    day07::part1(read_test_input(7));
    day07::part1(read_input(7));
    day07::part2(read_test_input(7));
    day07::part2(read_input(7));
    day08::part1(read_test_input(8));
    day08::part1(read_input(8));
    day08::part2(read_test_input(8));
    day08::part2(read_input(8));
    day09::part1(read_test_input(9));
    day09::part1(read_input(9));
    day09::part2(read_test_input(9));
    day09::part2(read_input(9));*/
    /*day10::part1(read_test_input(10));
    day10::part1(read_input(10));
    day10::part2(read_test_input(10));
    day10::part2(read_input(10));*/
    /*day11::part1(read_test_input(11));
    day11::part1(read_input(11));
    day11::part2(read_test_input(11));
    day11::part2(read_input(11));
    day12::part1(read_test_input(12));
    day12::part1(read_input(12));
    day12::part2(read_test_input(12));
    day12::part2(read_input(12));
    day13::part1(read_test_input(13));
    day13::part1(read_input(13));
    day13::part2(read_test_input(13));
    day13::part2(read_input(13));
    day14::part1(read_test_input(14));
    day14::part1(read_input(14));
    day14::part2(read_test_input(14));
    day14::part2(read_input(14));
    day15::part1(read_test_input(15));
    day15::part1(read_input(15));
    day15::part2(read_test_input(15));
    day15::part2(read_input(15));
    day16::part1(read_test_input(16));
    day16::part1(read_input(16));
    day16::part2(read_test_input(16));
    day16::part2(read_input(16));
    day17::part1(read_test_input(17));
    day17::part1(read_input(17));
    day17::part2(read_test_input(17));
    day17::part2(read_input(17));*/
    day18::part1(read_test_input(18));
    day18::part1(read_input(18));
    day18::part2(read_test_input(18));
    day18::part2(read_input(18));
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}

fn read_test_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/test{:0>2}.txt", day)).unwrap()
}