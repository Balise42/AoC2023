#[derive(Copy, Clone, Debug)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

fn intersect_hail_xy(h1: Hail, h2: Hail, pos1: f64, pos2: f64) -> usize {
    //h1.x + h1.vx * t = h2.x + h2.vx * s;
    //h1.y + h2.vy * t = h2.y + h2.vy * s;
    let a = h1.x;
    let b = h1.vx;
    let c = h2.x;
    let d = h2.vx;
    let e = h1.y;
    let f = h1.vy;
    let g = h2.y;
    let h = h2.vy;

    let t = ((g*d/h - c) - (e*d/h -a)) / (f*d/h -b);

    let x = h1.x + h1.vx * t;
    let y = h1.y + h1.vy * t;

    let s = (e + f*t - g) / h;

    if t >= 0.0 && s >= 0.0 && x >= pos1 && x <= pos2 && y >= pos1 && y <= pos2 {
        return 1;
    }
    return 0;
}

fn parse_hail(s: &str) -> Hail {
    let mut sides = s.split(" @ ");
    let mut coords = sides.next().unwrap().split(", ");
    let mut vel = sides.next().unwrap().split(", ");
    return Hail {
        x: coords.next().unwrap().trim().parse::<f64>().unwrap(),
        y: coords.next().unwrap().trim().parse::<f64>().unwrap(),
        z: coords.next().unwrap().trim().parse::<f64>().unwrap(),
        vx: vel.next().unwrap().trim().parse::<f64>().unwrap(),
        vy: vel.next().unwrap().trim().parse::<f64>().unwrap(),
        vz: vel.next().unwrap().trim().parse::<f64>().unwrap(),
    }
}

fn parallel(h1: Hail, h2: Hail)-> bool {
    let factor = h1.vx / h2.vx;
    return (h1.vy / h2.vy - factor).abs() < 0.01  && (h1.vz/h2.vz - factor).abs() < 0.01;
}

pub fn part1(s: String, pos1: f64, pos2: f64) {
    let mut hails: Vec::<Hail> = Vec::new();
    for line in s.lines() {
        hails.push(parse_hail(line));
    }
    let mut intersect = 0;
    for i in 0..hails.len() {
        for j in i+1..hails.len() {
            intersect += intersect_hail_xy(*hails.get(i).unwrap(), *hails.get(j).unwrap(), pos1, pos2);
        }
    }
    println!("{}", intersect)
}

pub fn part2(s: String) {
    let mut hails: Vec::<Hail> = Vec::new();
    for line in s.lines() {
        hails.push(parse_hail(line));
    }
    for i in 0..3 {
        let hail = hails.get(i).unwrap();
        println!("{} + {}*t{} == x + v1*t{} && ", hail.x, hail.vx, i, i);
        println!("{} + {}*t{} == y + v2*t{} && ", hail.y, hail.vy, i, i);
        println!("{} + {}*t{} == z + v3*t{} &&", hail.z, hail.vz, i, i);
    }
}
