use std::collections::VecDeque;

fn hash(s: &str) -> u32 {
    let chars = s.chars();
    let mut res = 0;
    for c in chars {
        res += u32::from(c);
        res *= 17;
        res %= 256;
    }
    return res;
}

pub fn part1(s: String) {
    let mut sum  = 0;
    for val in s.split(',') {
        sum += hash(val);
    }
    println!("{}", sum);
}

fn get_boxes(instrs: Vec::<&str>) -> Vec::<VecDeque::<(&str, u32)>> {
    let mut res: Vec::<VecDeque::<(&str, u32)>> = Vec::new();
    for i in 0..256 {
        res.push(VecDeque::new());
    }

    for instr in instrs {
        let mut chars: Vec::<char> = instr.chars().collect();
        if *chars.get(chars.len() - 1).expect("last") == '-' {
            let label = instr.get(0..instr.len()-1).expect("substr");
            let h = hash(label);
            let mut vec = &mut res[TryInto::<usize>::try_into(h).unwrap()];
            for i in 0..vec.len() {
                if vec.get(i).unwrap().0 == label {
                    vec.remove(i);
                    break;
                }
            }
        } else {
            let toks: Vec::<&str> = instr.split('=').collect();
            let label = *toks.get(0).unwrap();
            let val = toks.get(1).unwrap().parse::<u32>().unwrap();
            let h = hash(label);

            let mut vec = &mut res[TryInto::<usize>::try_into(h).unwrap()];

            let mut found = false;
            for i in 0..vec.len() {
                if vec.get(i).unwrap().0 == label {
                    vec[i].1 = val;
                    found = true;
                    break;
                }
            }
            if !found {
                vec.push_back((label, val));
            }
        }
    }
    return res;
}

pub fn part2(s: String) {
    let instrs = s.split(',').collect();
    let boxes = get_boxes(instrs);
    let mut sum: u32 = 0;
    for i in 0..boxes.len() {
        let mut mul: u32 = 1;
        let b = boxes.get(i).unwrap();
        let boxindex: u32 = (i+1).try_into().unwrap();
        for j in 0..b.len() {
            let lens : u32 = b.get(j).unwrap().1;
            let lensindex: u32 = (j+1).try_into().unwrap();
            sum += boxindex * lens * lensindex;
        }
    }
    println!("{}", sum);
}