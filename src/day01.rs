fn get_first_digit( l: &str ) -> u32 {
    for c in l.chars() {
        match c.to_digit(10) {
            Some(v) => {return v},
            None => {continue},
        }
    }
    return 0;
}

fn get_last_digit( l: &str ) -> u32 {
    return get_first_digit(&l.chars().rev().collect::<String>());
}

pub fn part1(s: String) {
    let lines = s.lines();
    let mut sum = 0;
    for line in lines {
        let d1 = get_first_digit(line);
        let d2 = get_last_digit(line);
        sum += d1*10 + d2;
    }
    println!("{}", sum);
}

fn get_all_digits( l: &str ) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    let mut i = 0;
    let chars: Vec<char> = l.chars().collect();
    while i < chars.len() {
        match chars[i] {
            '0'..='9' => { 
                res.push( chars[i].to_digit(10).unwrap());
                i+=1;
            },
            'o' => {
                if chars.len() >= i+3 && String::from_iter(&chars[i..i+3]) == "one" {
                    res.push(1);
                    // eight starts with e
                    i+=2;
                } else {
                    i+=1;
                }

            },
            't' => {
                if chars.len() >= i+3 && String::from_iter(&chars[i..i+3]) == "two" {
                    res.push(2);
                    // one starts with o
                    i+=2;
                } else if chars.len() >= i+5 && String::from_iter(&chars[i..i+5]) == "three" {
                    res.push(3);
                    // eight starts with e
                    i+=4;
                } else {
                    i+=1;
                }
            },
            'f' => {
                if chars.len() >= i+4 && String::from_iter(&chars[i..i+4]) == "four" {
                    res.push(4);
                    i+=4;
                } else if chars.len() >= i+4 && String::from_iter(&chars[i..i+4]) == "five" { 
                    res.push(5);
                    // eight starts with e
                    i+=3;
                } else {
                    i+=1;
                }
            },
            's' => {
                if chars.len() >= i+3 && String::from_iter(&chars[i..i+3]) == "six" {
                    res.push(6);
                    i+=3;
                } else if chars.len() >= i+5 && String::from_iter(&chars[i..i+5]) == "seven" { 
                    res.push(7);
                    // nine starts with n
                    i+=4;
                } else {
                    i+=1;
                }
            },
            'e' => {
                if chars.len() >= i+5 && String::from_iter(&chars[i..i+5]) == "eight" { 
                    res.push(8);
                    // two and three starts with t
                    i+=4;
                } else {
                    i+=1;
                }
            },
            'n' => {
                if chars.len() >= i+4 && String::from_iter(&chars[i..i+4]) == "nine" { 
                    res.push(9);
                    // eight starts with e
                    i+=3;
                } else {
                    i+=1;
                }
            },
            _ => { i+=1 }
        };
    }
    return res;
}

#[test]
fn test_get_all_digits() {
    assert_eq!(get_all_digits("0onetwothreefourfive4sixseveneightnineightwo"), [0,1,2,3,4,5,4,6,7,8,9,8,2]);
}

pub fn part2(s: String) {
    let lines = s.lines();
    let mut sum = 0;
    for line in lines {
        let digs = get_all_digits(line);
        sum += digs[0]*10 + digs.last().unwrap();
    }
    println!("{}", sum);
}