use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Block {
    id: usize,
    oxmin: usize,
    oxmax: usize,
    oymin: usize,
    oymax: usize,
    ozmin: usize,
    ozmax: usize,
    dzmin: Option::<usize>,
    dzmax: Option::<usize>,
}

impl Ord for Block {
    fn cmp(&self, other: &Block) -> Ordering {
        return (self.ozmin, self.oxmin, self.oymin).cmp(&(other.ozmin, other.oxmin, other.oymin));
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Block) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_block(s: &str, i: usize) -> Block {
    let mut coords = s.split('~');
    let mut min = coords.next().unwrap().split(',');
    let mut max = coords.next().unwrap().split(',');
    let xmin = min.next().unwrap().parse::<usize>().unwrap();
    let ymin = min.next().unwrap().parse::<usize>().unwrap();
    let zmin = min.next().unwrap().parse::<usize>().unwrap();
    let xmax = max.next().unwrap().parse::<usize>().unwrap();
    let ymax = max.next().unwrap().parse::<usize>().unwrap();
    let zmax = max.next().unwrap().parse::<usize>().unwrap();
    let block = Block {
        id: i,
        oxmin: xmin, oymin: ymin, ozmin: zmin,
        oxmax: xmax, oymax: ymax, ozmax: zmax,
        dzmin: None,
        dzmax: None,
    };
    return block;
}

fn dummy_block(i: usize) -> Block {
    let xmin = 8000;
    let ymin = 8000;
    let zmin = 8000;
    let xmax = 8000;
    let ymax = 8000;
    let zmax = 8000;
    let block = Block {
        id: i,
        oxmin: xmin, oymin: ymin, ozmin: zmin,
        oxmax: xmax, oymax: ymax, ozmax: zmax,
        dzmin: None,
        dzmax: None,
    };
    return block;
}

fn drop_block(b: &mut Block, grid: &mut HashMap <(usize, usize), (usize, Vec::<usize>)>) {
    let mut height: usize = 0;
    for x in b.oxmin..=b.oxmax {
        for y in b.oymin..=b.oymax {
            let (z, _) = grid.entry((x, y)).or_insert((1, Vec::new()));
            height = std::cmp::max(*z, height);
        }
    }
    b.dzmin = Some(height);
    b.dzmax = Some(height + (b.ozmax - b.ozmin));
    for x in b.oxmin..=b.oxmax {
        for y in b.oymin..=b.oymax {
            grid.entry((x,y)).and_modify(|x| {x.0 = b.dzmax.unwrap() + 1; x.1.push(b.id)});
        }
    }
}

fn direct_neighbors(block: &Block, grid: &HashMap::<(usize, usize), (usize, Vec::<usize>)>, blocks_by_id: &Vec::<Block>) -> (HashSet::<usize>, HashSet::<usize>){
    let mut below: HashSet<usize> = HashSet::new();
    let mut above: HashSet<usize> = HashSet::new();
    for x in block.oxmin..=block.oxmax {
        for y in block.oymin..=block.oymax {
            let (_, col) = grid.get(&(x, y)).unwrap();
            let index = col.iter().position(|x| x == &block.id).unwrap();
            if index > 0 {
                let bblock = blocks_by_id.get(*col.get(index-1).unwrap()).unwrap();
                if bblock.dzmax.unwrap() + 1 == block.dzmin.unwrap() {
                    below.insert(bblock.id);
                }
            }
            if index < col.len()-1 {
                let ablock = blocks_by_id.get(*col.get(index+1).unwrap()).unwrap();
                if block.dzmax.unwrap() + 1 == ablock.dzmin.unwrap() {
                    above.insert(ablock.id);
                }
            }
        }
    }
    return (below, above);
}

pub fn part1(s: String) {
    let mut i = 0;
    let mut blocks: Vec::<Block> = Vec::new();
    for line in s.lines() {
        blocks.push(parse_block(line, i));
        i+=1;
    }
    let mut blocks_by_id = blocks.clone();
    blocks.sort();
    let mut grid : HashMap::<(usize, usize), (usize, Vec::<usize>)> = HashMap::new();
    for mut block in &mut blocks {
        drop_block(&mut block, &mut grid);
        blocks_by_id[block.id] = block.clone()
    }
    let mut below_map: HashMap::<usize, HashSet::<usize>> = HashMap::new();
    let mut above_map: HashMap::<usize, HashSet::<usize>> = HashMap::new();

    for block in &blocks {
        let (below, above) =  direct_neighbors(&block, &grid, &blocks_by_id);
        below_map.insert(block.id, below);
        above_map.insert(block.id, above);
    }

    let mut sum = 0;

    for block in &blocks {
        let mut deletable = true;
        for ab in above_map.get(&block.id).unwrap() {
            if below_map.get(ab).unwrap().len() == 1 {
                deletable = false;
                break;
            }
        }
        if deletable {
            sum += 1;
        }
    }
    println!("{}", sum);
}

pub fn part2(s: String) {
    let mut i = 0;
    let mut blocks: Vec::<Block> = Vec::new();
    for line in s.lines() {
        blocks.push(parse_block(line, i));
        i+=1;
    }
    let mut orig_blocks_by_id = blocks.clone();
    blocks.sort();
    let mut grid : HashMap::<(usize, usize), (usize, Vec::<usize>)> = HashMap::new();
    for mut block in &mut blocks {
        drop_block(&mut block, &mut grid);
        orig_blocks_by_id[block.id] = block.clone()
    }
    let mut below_map: HashMap::<usize, HashSet::<usize>> = HashMap::new();
    let mut above_map: HashMap::<usize, HashSet::<usize>> = HashMap::new();

    for block in &blocks {
        let (below, above) =  direct_neighbors(&block, &grid, &orig_blocks_by_id);
        below_map.insert(block.id, below);
        above_map.insert(block.id, above);
    }

    let mut to_process: Vec<usize> = Vec::new();

    for block in &blocks {
        let mut deletable = true;
        for ab in above_map.get(&block.id).unwrap() {
            if below_map.get(ab).unwrap().len() == 1 {
                deletable = false;
                break;
            }
        }
        if !deletable {
            to_process.push(block.id);
        }
    }
    
    let mut sum = 0;

    for id in to_process {
        let mut i = 0;
        let mut blocks: Vec::<Block> = Vec::new();
        for line in s.lines() {
            if (i != id) {
                blocks.push(parse_block(line, i));
            } else {
                blocks.push(dummy_block(i));
            }
            i+=1;
        }
        let mut blocks_by_id = blocks.clone();
        blocks.sort();
        let mut grid : HashMap::<(usize, usize), (usize, Vec::<usize>)> = HashMap::new();
        for mut block in &mut blocks {
            drop_block(&mut block, &mut grid);
            blocks_by_id[block.id] = block.clone()
        }

        for b in 0..blocks_by_id.len() {
            if blocks_by_id.get(b).unwrap() != orig_blocks_by_id.get(b).unwrap() {
                sum += 1;
            }
        }
        sum -=1;
    }
    println!("{}", sum);

}