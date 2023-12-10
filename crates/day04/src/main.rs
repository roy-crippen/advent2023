fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let cards = parse(&lines);
    // for card in &cards {
    //     println!("{card:?}")
    // }

    println!("day01-a = {}", solve_a(&cards)); // 54632
}

#[derive(Debug)]
struct Card {
    pub id: u32,
    pub wins: Vec<u32>,
    pub attempts: Vec<u32>,
}

fn solve_a(cards: &[Card]) -> u32 {
    let get_value = |card: &Card| -> u32 {
        let cnt: u32 = card.attempts.iter().map(|v| if card.wins.contains(v) { 1 } else { 0 }).sum();
        if cnt == 0 {
            return 0;
        }
        2u32.pow(cnt - 1)
    };
    cards.iter().map(get_value).sum()
}

/// Parse input `lines` to create a list of Cards
fn parse(lines: &[String]) -> Vec<Card> {
    let get_vals = |in_str: &str| -> Vec<u32> {
        in_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<u32>().unwrap_or(0u32))
            .collect()
    };
    let mut cards: Vec<Card> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let xs: Vec<&str> = line.split_once(':').unwrap().1.split('|').map(|s| s.trim()).collect();
        let wins: Vec<u32> = get_vals(xs[0]);
        let attempts: Vec<u32> = get_vals(xs[1]);
        cards.push(Card { id: i as u32 + 1, wins, attempts })
    }
    cards
}
