use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Coords {
    line: i32,
    col: i32
}

fn parse_pieces(s: String) -> (HashMap<Coords, i32>, HashMap<Coords, char>) {
    let mut line = 0;
    let mut col = 0;
    let mut num = -1;
    let mut pieces: HashMap<Coords, i32> = HashMap::new();
    let mut symbols: HashMap<Coords, char> = HashMap::new();
    for c in s.chars() {
        match c {
            '.' => { 
                if num != -1 {
                    pieces.insert(Coords{line: line, col: col-1}, num );
                    num = -1;
                }
                col += 1;
            }
            '0'..='9' => {
                let digit = c.to_digit(10).unwrap() as i32;
                if num != -1 {
                    num = num * 10 + digit;
                } else {
                    num = digit;
                }
                col += 1;
            }
            '\n' => {
                if num != -1 {
                    pieces.insert(Coords{line: line, col: col-1}, num );
                    num = -1;
                }
                col = 0;
                line += 1;
            }
            _ => { 
                if num != -1 {
                    pieces.insert(Coords{line: line, col: col-1}, num );
                    num = -1;
                }
                symbols.insert(Coords{line: line, col: col}, c); 
                col +=1;
            }
        };
    }
    if num != -1 {
        pieces.insert(Coords{line: line, col: col-1}, num );
    }
    return (pieces, symbols);
}

fn has_adjacent_symbol( symbols: HashMap<Coords, char>, key: Coords ) -> bool {
    return get_adjacent_symbols( symbols, key ).len() > 0;
}

fn get_adjacent_symbols( symbols: HashMap<Coords, char>, key: Coords ) -> HashMap<Coords, char> {
    let mut res: HashMap<Coords, char> = HashMap::new();
    for line in key.line-1..=key.line+1 {
        for col in key.col-1..=key.col+1 {
            match symbols.get( &Coords{line:line, col:col} ) {
                Some(s) => { res.insert(Coords{line:line, col:col}, *s); }
                None => {}
            };
        }
    }
    return res;
}

pub fn part1(s: String) {
    let (pieces, symbols) = parse_pieces(s);

    let mut sum = 0;
    for (key, value) in pieces {
        if has_adjacent_symbol(symbols.clone(), key.clone()) 
        || ( value >= 10 && has_adjacent_symbol( symbols.clone(), Coords{line:key.line, col:key.col-1}) )
        || ( value >= 100 && has_adjacent_symbol( symbols.clone(), Coords{line:key.line, col:key.col-2}) ) {
            sum += value;
        }
    }

    println!( "{}", sum);
}

pub fn part2(s: String) {
    let (pieces, symbols) = parse_pieces(s);
    let mut adj: HashMap<Coords, HashSet<i32>> = HashMap::new();
    
    for (key, value) in pieces {
        let mut to_check: Vec<Coords> = Vec::new();
        to_check.push(key.clone());
        if (value >= 10) {
            to_check.push(Coords{line: key.line, col: key.col-1});
        }
        if (value >= 100) {
            to_check.push(Coords{line: key.line, col: key.col-2});
        }
        for numcoord in to_check {
            for (coord, symb) in get_adjacent_symbols(symbols.clone(), numcoord.clone()) {
                if symb == '*' {
                    adj.entry(coord.clone()).or_insert(HashSet::new()).insert(value);
                }
            }
        }       
    }

    let mut sum = 0;
    for (key, value) in adj {
        if value.len() == 2 {
            let mut prod = 1;
            for val in value.iter() {
                prod *= val;
            }
            sum += prod;
        }
    }

    println!("{}", sum);
}