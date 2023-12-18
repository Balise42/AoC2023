pub fn part1(s: String) {
    let mut old: (i32, i32) = (0, 0);
    let mut area = 0;
    let mut path = 0;
    for l in s.lines() {
        let mut toks = l.split_whitespace();
        let dir = toks.next().expect("dir");
        let steps = toks.next().expect("steps").parse::<i32>().expect("parse");
        path += steps;
        let mut new = match dir {
            "R" => {
                (old.0, old.1 + steps)
            },
            "L" => {
                (old.0, old.1 - steps)
            },
            "U" => {
                (old.0 - steps, old.1)
            },
            "D" => {
                (old. 0 + steps, old.1)
            }
            _ => {
                old
            }

        };
        area += old.0 * new.1 - old.1*new.0;
        old = new;
    }
    println!("{}", -area/2 + path/2 + 1);
}

pub fn part2(s: String) {
    let mut old: (i64, i64) = (0, 0);
    let mut area = 0;
    let mut path = 0;
    for l in s.lines() {
        let chars: Vec::<char> = l.split_whitespace().last().expect("last").chars().collect();
        let dir = match *chars.get(7).expect("7") {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => ""
        };
        let steps: i64 = (chars.get(2).expect("2").to_digit(16).unwrap() * 65536
            + chars.get(3).expect("3").to_digit(16).unwrap() * 4096
            + chars.get(4).expect("4").to_digit(16).unwrap() * 256
            + chars.get(5).expect("5").to_digit(16).unwrap() * 16
            + chars.get(6).expect("6").to_digit(16).unwrap() * 1).try_into().unwrap();
        path += steps;
        let mut new = match dir {
            "R" => {
                (old.0, old.1 + steps)
            },
            "L" => {
                (old.0, old.1 - steps)
            },
            "U" => {
                (old.0 - steps, old.1)
            },
            "D" => {
                (old. 0 + steps, old.1)
            }
            _ => {
                old
            }

        };
        area += old.0 * new.1 - old.1*new.0;
        old = new;
    }
    println!("{}", -area/2 + path/2 + 1);
}