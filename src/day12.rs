use memoize::memoize;

fn parse(s: &str) -> (Vec::<char>, Vec::<i64>) {
    let toks: Vec<&str> = s.split_whitespace().collect();
    let springs = toks.get(0).expect("springs").chars().collect();
    let groups: Vec::<i64> = toks.get(1).expect("groups").split(',').map(|x| x.parse::<i64>().expect("num")).collect();
    return (springs, groups);
}

#[memoize]
fn num_comb(sp: Vec::<char>, gr: Vec::<i64>, currGroup: i64) -> i64 {

    let mut groups = gr.clone();
    let mut springs = sp.clone();

    if springs.len() == 0 {
        if (groups.len() == 1 && *groups.get(0).expect("group") == currGroup) {
            return 1;
        }
        return if groups.len() > 0 {0} else {1};
    }

    let newvec = springs.clone().split_off(1);
    
    if (groups.len() == 0) {
        if currGroup == 0 && *springs.get(0).expect("spring") != '#' {
            return num_comb(newvec.clone(), groups, 0);
        } else {
            return 0;
        }
    }

    let nextGroup = *groups.get(0).expect("group");
    if currGroup > nextGroup {
        return 0;
    }
    let nextSpring = springs.get(0).expect("spring");

    match nextSpring {
        '?' => {
            let mut sum = 0;
            if ( currGroup == 0 ) {
                // consider whatevs
                sum += num_comb(newvec.clone(), groups.clone(), currGroup + 1) + num_comb(newvec.clone(), groups, 0);
            } else if ( currGroup < nextGroup ) {
                // consider it being '#'
                sum += num_comb(newvec.clone(), groups.clone(), currGroup + 1);
            } else {
                // consider it being '.'
                if currGroup != 0 {
                    if nextGroup == currGroup {
                        sum += num_comb(newvec, groups.split_off(1), 0);
                    }
                } else {
                    sum += num_comb(newvec, groups, 0)
                }
            }
            return sum;
        },
        '#' => {
            return num_comb(newvec, groups, currGroup + 1);
        },
        '.' => {
            if currGroup != 0 {
                if nextGroup == currGroup {
                    return num_comb(newvec, groups.split_off(1), 0);
                } else {
                    return 0;
                }
            }
            return num_comb(newvec, groups, 0)
        },
        _ => { return 0; }
    };
}

pub fn part1(s: String) {
    let mut sum = 0;
    for line in s.lines() {
        let (springs, groups) = parse(line);
        let num = num_comb(springs, groups, 0);
        sum += num;
    }
    println!("{}", sum);
}

pub fn part2(s: String) {
    let mut sum = 0;
    for line in s.lines() {
        let (springs, groups) = parse(line);
        let mut allsprings: Vec::<char> = Vec::new();
        let mut allgroups: Vec::<i64> = Vec::new();
        for i in 0..4 {
            allsprings.append(&mut springs.clone());
            allsprings.push('?');
            allgroups.append(&mut groups.clone());
        }
        allsprings.append(&mut springs.clone());
        allgroups.append(&mut groups.clone());
        let num = num_comb(allsprings, allgroups, 0);
        sum += num;
    }
    println!("{}", sum);
}