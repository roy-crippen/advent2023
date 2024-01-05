use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let map = parse(&lines);

    println!("day08-a = {}", solve_a(&map)); // 16897
}

struct Map {
    m: HashMap<String, (String, String)>,
    rev_instructions: String,
}

fn solve_a(map: &Map) -> usize {
    let mut current_key = "AAA";
    let mut rev_instructions: String = map.rev_instructions.clone();
    let mut steps = 0;
    loop {
        steps += 1;
        if rev_instructions.is_empty() {
            rev_instructions = map.rev_instructions.clone();
        }
        let (l_key, r_key) = map.m.get(current_key).unwrap();
        current_key = if rev_instructions.pop().unwrap().eq(&'L') { l_key } else { r_key };
        if current_key.eq("ZZZ") {
            return steps;
        }
    }
}

fn parse(lines: &[String]) -> Map {
    assert!(lines.len() > 2);
    let mut it = lines.iter();
    let rev_instructions = it.next().unwrap().chars().rev().collect();
    it.next();
    let mut m: HashMap<String, (String, String)> = HashMap::new();
    for line in it {
        let (key, s) = line.split_once('=').unwrap();
        let (ls, rs) = s.split_once(',').unwrap();
        let ls = ls.trim();
        let rs = rs.trim();
        m.insert(key.trim().to_string(), (ls[1..].to_string(), rs[..rs.len() - 1].to_string()));
    }

    Map { m, rev_instructions }
}
