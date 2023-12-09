fn parse_ints( s: &str ) -> Vec<i64> {
    s.split_whitespace()
    .map(|x| x.parse::<i64>().expect("num"))
    .collect()
}

fn next_value(line: &str) -> (i64, i64) {
    let mut vals = parse_ints(line);
    let mut numiters = 0;
    let mut lasts: Vec::<i64> = Vec::new();
    let mut firsts: Vec::<i64> = Vec::new();
    lasts.push(*vals.get(vals.len()-1).unwrap());
    firsts.push(*vals.get(0).unwrap());

    loop {
        let mut tmp: Vec::<i64> = Vec::new();
        for i in 0..vals.len() - 1 {
            tmp.push(*vals.get(i+1).unwrap() - *vals.get(i).unwrap());
        }
        lasts.push(*tmp.get(tmp.len()-1).unwrap());
        firsts.push(*tmp.get(0).unwrap());
        let mut cont = false;
        for val in &tmp {
            if *val != 0 {
                cont = true;
                break;
            }
        }
        vals = tmp;
        if !cont {
            break;
        }
    }
    let mut resfirst = 0;
    for i in 1..=firsts.len() {
        resfirst = *firsts.get(firsts.len()-i).unwrap() - resfirst;
    }
    return (lasts.iter().sum(), resfirst);
}

pub fn part1(s: String) {
    let mut sum = 0;
    for line in s.lines() {
        let (last, _first) = next_value(line);
        sum += last;
    }
    println!("{}", sum);
}

pub fn part2(s: String) {
    let mut sum = 0;
    for line in s.lines() {
        let (_last, first) = next_value(line);
        sum += first;
    }
    println!("{}", sum);
}