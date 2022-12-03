fn main() {
    prob1();
    prob2();
}

fn prob1() {
    let score = include_str!("input.txt")  
                    .lines()
                    .map(|l| l.split_once(" ").unwrap())
                    .fold(0, | score, (move1, move2) | {
                        match (move1, move2) {
                            // rock A X
                            // paper B Y
                            // scissors C Z
                            ("A", "X") => score + 1 + 3,
                            ("A", "Y") => score + 2 + 6,
                            ("A", "Z") => score + 3 + 0,
                            ("B", "X") => score + 1 + 0,
                            ("B", "Y") => score + 2 + 3,
                            ("B", "Z") => score + 3 + 6,
                            ("C", "X") => score + 1 + 6,
                            ("C", "Y") => score + 2 + 0,
                            ("C", "Z") => score + 3 + 3,
                            _ => 0
                        }
                    });

    println!("The score would be '{score}' for prob 1");
}

fn prob2() {
    let score = include_str!("input.txt")  
                    .lines()
                    .map(|l| l.split_once(" ").unwrap())
                    .fold(0, | score, (move1, outcome) | {
                        match (move1, outcome) {
                            // X  lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
                            ("A", "X") => score + 3 + 0,
                            ("A", "Y") => score + 1 + 3,
                            ("A", "Z") => score + 2 + 6,
                            ("B", "X") => score + 1 + 0,
                            ("B", "Y") => score + 2 + 3,
                            ("B", "Z") => score + 3 + 6,
                            ("C", "X") => score + 2 + 0,
                            ("C", "Y") => score + 3 + 3,
                            ("C", "Z") => score + 1 + 6,
                            _ => 0
                        }
                    });

    println!("The score would be '{score}' for prob 2");
}