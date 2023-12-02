struct Day2Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_game( s: &str ) -> Day2Game {
    let mut toks = s.split(": ");
    let id: u32 = toks.next().unwrap().split(' ').last().unwrap().parse().unwrap();
    
    let sets = toks.next().unwrap().split("; ");

    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    for set in sets {
        let mut rep = set.split(", ");
        for tok in rep {
            let mut numcol = tok.split(' ');
            let num: u32 = numcol.next().unwrap().parse().unwrap();
            let col = numcol.next().unwrap();
            if col == "red" && num > red {
                red = num;
            } else if col == "blue" && num > blue  {
                blue = num;
            } else if col == "green" && num > green {
                green = num;
            }
        }
    }

    let game = Day2Game {
        id: id,
        red: red,
        green: green,
        blue: blue
    };
    game
}

pub fn part1(s: String) {
    let lines = s.lines();
    let mut sum = 0;
    for line in lines {
        let game = parse_game(line);
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            sum += game.id;
        }
    }
    println!("{}", sum);
}

pub fn part2(s: String) {
    let lines = s.lines();
    let mut sum = 0;
    for line in lines {
        let game = parse_game(line);
        let power = game.green * game.red * game.blue;
        sum += power;
    }
    println!("{}", sum);
}