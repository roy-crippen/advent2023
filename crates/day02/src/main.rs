fn main() {
    let ss: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|l| l.to_owned())
        .collect();
    let games = parse(&ss);
    let (va, vb) = solve(&games, (12, 13, 14));
    println!("day02-a = {va}"); // 2679
    println!("day02-b = {vb}") // 77607
}

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub cubes: Vec<(u32, u32, u32)>,
}

fn solve(games: &Vec<Game>, c: (u32, u32, u32)) -> (u32, u32) {
    let mut va: u32 = 0;
    let mut vb: u32 = 0;
    for game in games {
        let mut a_fail = false;
        let mut b_max: (u32, u32, u32) = (0, 0, 0);
        for cube in &game.cubes {
            b_max = (
                b_max.0.max(cube.0),
                b_max.1.max(cube.1),
                b_max.2.max(cube.2),
            );
            if cube.0 > c.0 || cube.1 > c.1 || cube.2 > c.2 {
                a_fail = true;
            }
        }
        if !a_fail {
            va += game.id
        }
        vb += b_max.0 * b_max.1 * b_max.2
    }
    (va, vb)
}

fn parse(ss: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::with_capacity(ss.len());
    for (i, s) in ss.iter().enumerate() {
        let _s = s.split_once(':').unwrap().1;
        let xss: Vec<Vec<(u32, &str)>> = _s
            .split(';')
            .map(|s| {
                s.split(',')
                    .map(|s| {
                        let vs: Vec<&str> = s.trim().split(' ').collect();
                        let n = vs[0].parse::<u32>().unwrap();
                        let c = vs[1];
                        (n, c)
                    })
                    .collect::<Vec<(u32, &str)>>()
            })
            .collect();
        let mut game: Game = Game {
            id: i as u32 + 1,
            cubes: Vec::new(),
        };
        for xs in &xss {
            let mut cube = (0, 0, 0);
            for (n, c) in xs {
                match *c {
                    "red" => cube.0 = *n,
                    "green" => cube.1 = *n,
                    _ => cube.2 = *n,
                }
            }
            game.cubes.push(cube);
        }
        // println!("{game:?}");
        games.push(game);
    }
    games
}
