fn beat_record(time: &u64, dist: &u64) -> u64 {
    let mut sum = 0;
    for i in 0..=*time {
        let curr = i * (time-i);
        if curr > *dist {
            sum+=1;
        }
    }
    sum
}

#[test]
fn test_beat_record() {
    assert_eq!(4, beat_record(&7, &9));
    assert_eq!(8, beat_record(&15, &40));
    assert_eq!(9, beat_record(&30, &200));
}

pub fn part1(s: String) {
    let lines: Vec::<&str> = s.lines().collect();
    let times: Vec::<u64> = lines.first().expect("times").split_whitespace()
        .filter(|x| x.chars().next().expect("char").is_numeric())
        .map(|x| x.parse::<u64>().expect("time")).collect();
    let dists: Vec::<u64> = lines.last().expect("dists").split_whitespace()
        .filter(|x| x.chars().next().expect("char").is_numeric())
        .map(|x| x.parse::<u64>().expect("dist")).collect();

    let mut res = 1;
    for i in 0..times.len() {
        let rec = beat_record(times.get(i).expect("time"), dists.get(i).expect("dist"));
        res *= rec;
    }
    println!("{}", res);
}

pub fn part2(s: String) {
    let lines: Vec::<&str> = s.lines().collect();
    let time = lines.get(0).expect("time").replace(" ", "")
        .split(':').last().expect("time").parse::<u64>().expect("time");
    let dist = lines.get(1).expect("dist").replace(" ", "")
        .split(':').last().expect("dist").parse::<u64>().expect("dist");
    println!("{}", beat_record(&time, &dist));
}