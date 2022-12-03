use std::collections::HashMap;

fn main() {

    let mut scores : HashMap<char, usize> = HashMap::new();
    ('a'..='z').chain('A'..='Z').enumerate().for_each(|(i, c)| {
        scores.insert(c, i);
    });

    part1(&scores);
    part2(&scores);

}

fn part1(scores: &HashMap<char, usize>) {
    let score = include_str!("input.txt")  
                    .lines()
                    .fold(0 , | score, l | {
                        let (w1, w2) = l.split_at(l.len()/2);

                        for ch in w1.chars() {
                            if w2.contains(ch) {
                                return score + *scores.get(&ch).unwrap();
                            }
                        }
                        return 0;
                    });

                   
    println!("The score would be '{score}' for part 1");
}

fn part2(scores: &HashMap<char, usize>) {
    let score = include_str!("input.txt")
                .split("\n")
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
                .chunks(3)
                .fold(0, |score, g| {
                    let g1 = &g[0];
                    let g2 = &g[1];
                    let g3 = &g[2];
                    
                    for ch in g1 {
                        if g2.contains(&ch) && g3.contains(&ch) {
                            return score + *scores.get(&ch).unwrap();
                        }
                    }
                    return 0;
                }
    );
    
    println!("The score would be '{score}' for part 2");
}


