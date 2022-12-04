
fn main() {

    part1();
    part2();

}

fn part1() {
    let score = include_str!("input.txt")  
                    .lines()
                    .map(|l| l.split_once(",").unwrap())
                    .fold(0 , | score, (r1,r2) | {
                        let (a,b) = r1.split_once("-").unwrap(); 
                        let (c,d) = r2.split_once("-").unwrap(); 

                        let a = a.parse::<i32>().unwrap();
                        let b = b.parse::<i32>().unwrap();
                        let c = c.parse::<i32>().unwrap();
                        let d = d.parse::<i32>().unwrap();

                        if (a >= c && b <= d) || (c >= a && d <= b) {
                            return score + 1;
                        }
                        score
                    });

                   
    println!("The score would be '{score}' for part 1");
}

fn part2() {
    let score = include_str!("input.txt")  
                    .lines()
                    .map(|l| l.split_once(",").unwrap())
                    .fold(0 , | score, (r1,r2) | {
                        let (a,b) = r1.split_once("-").unwrap(); 
                        let (c,d) = r2.split_once("-").unwrap(); 

                        let a = a.parse::<i32>().unwrap();
                        let b = b.parse::<i32>().unwrap();
                        let c = c.parse::<i32>().unwrap();
                        let d = d.parse::<i32>().unwrap();

                        let r1 = a..=b;
                        let r2 = c..=d;

                        if (a >= c && b <= d) || (c >= a && d <= b) {
                            return score + 1;
                        } else if r1.contains(&c) || r1.contains(&d) || r2.contains(&a) || r2.contains(&b) {
                            return score + 1;
                        }
                        score
                    });

                   
    println!("The score would be '{score}' for part 2");
}