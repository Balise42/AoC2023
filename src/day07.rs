use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, PartialEq,Debug)]
struct Hand {
    hand: String,
    bid: i32,
    part1: bool
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn hand_strength(hand: &Hand) -> i32 {
    let v: Vec<char> = hand.hand.chars().collect();
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in v {
        counts.entry(c).or_insert(0);
        counts.insert(c, counts.get(&c).unwrap() + 1);
    }
    if counts.len() == 1 {
        // five of a kind
        return 7;
    }
    if counts.len() == 2 {
        if !hand.part1 && counts.contains_key(&'J') {
            return 7;
        }
        for (_, v) in counts.iter() {
            if *v == 4 {
                // four of a kind
                return 6;
            }
        }
        // full house
        return 5;
    }
    if counts.len() == 3 {
        for (_, v) in counts.iter() {
            if *v == 3 {
                if !hand.part1 && counts.contains_key(&'J') {
                    return 6;
                }
                // three of a kind
                return 4;
            }
        }
        // two pairs
        if !hand.part1 && counts.contains_key(&'J') {
            if *(counts.get(&'J').unwrap()) == 2 {
                return 6;
            }
            return 5;
        }
        return 3;
    }
    if counts.len() == 4 {
        if !hand.part1 && counts.contains_key(&'J') {
            return 4;
        }
        return 2;
    }
    if counts.len() == 5 {
        if !hand.part1 && counts.contains_key(&'J') {
            return 2;
        }
        return 1;
    }
    return 0;
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let s1 = hand_strength(self);
        let s2 = hand_strength(other);
        if s1 != s2 {
            return s1.cmp(&s2);
        }
        let h1: Vec<char> = self.hand.chars().collect();
        let h2: Vec<char> = other.hand.chars().collect();
        for i in 0..h1.len() {
            let c1 = h1.get(i).expect("card1");
            let c2 = h2.get(i).expect("card2");
            if c1 != c2  {
                return card_value(*c1, self.part1).cmp(&card_value(*c2, self.part1));
            }
        }
        return Ordering::Equal;
    }
}

fn card_value(c: char, part1: bool) -> i32{
    match c {
        '2'..='9' => c.to_digit(10).expect("card").try_into().expect("sign"),
        'T' => 10,
        'J' => if part1 { 11 } else { 1 },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}



fn parse_hand(line: &str, part1: bool) -> Hand {
    let mut toks = line.split_whitespace();
    let hand = toks.next().expect("hand");
    let bid = toks.next().expect("bidstr").parse::<i32>().expect("bid");
    return Hand{ hand: hand.to_string(), bid: bid, part1: part1};
}

fn solve(s: String, part1: bool) {
    let lines = s.lines();
    let mut hands: Vec::<Hand> = Vec::new();
    for line in lines {
        let hand = parse_hand(line, part1);
        hands.push(hand);
    }
    hands.sort();
    let mut sum: i32 = 0;
    for i in 0..hands.len() {
        sum += TryInto::<i32>::try_into((i+1)).unwrap() * hands.get(i).expect("hand").bid;
    }
    println!("{}", sum);
}

pub fn part1(s: String) {
    solve(s, true);
}

pub fn part2(s: String) {
    solve(s, false);
}