use regex::Regex;
use std::collections::HashSet;

fn parse_cards(s: &str) -> (usize, HashSet<u32>, HashSet<u32>) {
    let re = Regex::new(r"Card\s+(\d+):([\d\s]+) \|([\d\s]+)").unwrap();
    let caps = re.captures(s).unwrap();

    let mut set1:HashSet<u32> = HashSet::new();
    for val in caps.get(2).unwrap().as_str().split_whitespace() {
        set1.insert( val.parse::<u32>().unwrap());
    }
    let mut set2:HashSet<u32> = HashSet::new();
    for val in caps.get(3).unwrap().as_str().split_whitespace() {
        set2.insert( val.parse::<u32>().unwrap());
    }
    return (caps.get(1).unwrap().as_str().parse::<usize>().unwrap(), set1, set2);
}

pub fn part1(s: String) {
    let lines = s.lines();
    let mut sum = 0;
    let base: u32 = 2;
    for line in lines {
        let (id, winning, have) = parse_cards(line);
        let count: u32 = winning.intersection(&have).count().try_into().unwrap();
        if count > 0 {
            sum += base.pow( count - 1 );
        }
    }
    println!("{}", sum);
}

pub fn part2(s: String) {
    let lines = s.lines();
    let mut sum = 0;
    let mut num_cards: [usize; 300] = [1; 300];
    for line in lines {
        sum += 1;
        let (id, winning, have) = parse_cards(line);
        let count: usize = winning.intersection(&have).count().try_into().unwrap();
        sum += count * (num_cards[id]);
        let mut i = id+1;
        let last_card: usize  = count + id + 1;
        while i < last_card.try_into().unwrap() {
            num_cards[i]+=(num_cards[id]);
            i += 1;
        }
    }
    println!("{}", sum);
   
}