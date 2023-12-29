fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let game = parse(&lines);
    println!("day05-a = {}", solve(&game.a_seeds, &game.mss)); // 174137457
    println!("day05-b = {}", solve(&game.b_seeds, &game.mss)); // 1493866
}

#[derive(Clone, Debug)]
struct Map {
    d_start: usize,
    s_start: usize,
    len: usize,
}

#[derive(Debug)]
struct Game {
    a_seeds: Vec<(usize, usize)>,
    b_seeds: Vec<(usize, usize)>,
    mss: Vec<Vec<Map>>,
}

/// solve both part a and b
fn solve(_seeds: &[(usize, usize)], mss: &Vec<Vec<Map>>) -> usize {
    // seed list to mutate
    let mut seeds = _seeds.to_vec();

    // loop the groups of of maps
    for ms in mss {
        let mut new_seeds: Vec<(usize, usize)> = Vec::new();
        while let Some((start_seed, end_seed)) = seeds.pop() {
            // apply each map to the current seed
            let mut has_no_overlap = true;
            for &Map { s_start, d_start, len } in ms {
                // find the overlap for this map
                let start_overlap = start_seed.max(s_start);
                let end_overlap = end_seed.min(s_start + len);

                // mutate new_seed and seed if there is overlap
                if start_overlap < end_overlap {
                    let new_start = d_start + start_overlap - s_start;
                    let new_end = d_start + end_overlap - s_start;
                    new_seeds.push((new_start, new_end));

                    if start_seed < start_overlap {
                        seeds.push((start_seed, start_overlap));
                    }

                    if end_overlap < end_seed {
                        seeds.push((end_overlap, end_seed));
                    }

                    // we handled the overlap, no more to do on this map
                    has_no_overlap = false;
                    break;
                }
            }
            // if there was no overlap
            if has_no_overlap {
                new_seeds.push((start_seed, end_seed));
            }
        }
        // update seed after applying the group of maps
        seeds = new_seeds;
    }

    // println!("seeds: {:?}", &seeds);
    seeds.iter().map(|(v, _)| *v).min().unwrap()
}

/// Parse input `lines` to create a Game
fn parse(lines: &[String]) -> Game {
    let mut it = lines.iter();

    // parse the seeds for both part a and b
    let a_seeds: Vec<(usize, usize)> = it
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| {
            let v = s.trim().parse::<usize>().unwrap_or(0);
            (v, v + 1)
        })
        .collect();
    let b_seeds = (0..a_seeds.len() - 1).step_by(2).map(|i| (a_seeds[i].0, a_seeds[i].0 + a_seeds[i + 1].0)).collect();

    // parse the maps
    let mut mss: Vec<Vec<Map>> = Vec::new();
    let mut ms: Vec<Map> = Vec::new();
    for line in it {
        if !line.is_empty() {
            if line.chars().next().unwrap().is_ascii_digit() {
                let xs: Vec<usize> = line.split(' ').map(|s| s.trim().parse::<usize>().unwrap_or(0)).collect();
                let m = Map { d_start: xs[0], s_start: xs[1], len: xs[2] };
                ms.push(m);
            }
        } else if !ms.is_empty() {
            mss.push(ms);
            ms = Vec::new();
        }
    }
    mss.push(ms);
    Game { a_seeds, b_seeds, mss }
}
