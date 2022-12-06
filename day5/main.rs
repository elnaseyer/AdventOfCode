
fn main() {
    prob1();
    prob2();
}
fn prob2() {
    let (stack, moves) = include_str!("input.txt").split_once("\n\n").unwrap();
    
    let mut stacks = stack.lines()
                    .rev()
                    .skip(1)
                    .fold(vec![Vec::new(); 9], |mut vec, line| {
                        for idx in (0..line.len()).step_by(4) {
                            let val = &line[idx+1..idx+2];
                            if val != " " { vec[idx/4].push(val) };
                        } 
                        vec
                    });
               
    moves.split("\n")
         .for_each(|w| {
            let mov: Vec<usize> = w.split(" ")
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect();
            let (num, from, to) = (mov[0], mov[1]-1, mov[2]-1);         
            let mut temp: Vec<&str> = Vec::new();
            for _ in 0..num {
                let popped = stacks[from].pop().unwrap();
                temp.push(popped);
            }       
            for elem in temp.iter().rev() {
                stacks[to].push(elem);
            }

         });

    for mut a in stacks {
        let val = a.pop().unwrap();
            print!("{}", val);
    }
}


fn prob1() {
    let (stack, moves) = include_str!("input.txt").split_once("\n\n").unwrap();
    
    let mut stacks = stack.lines()
                    .rev()
                    .skip(1)
                    .fold(vec![Vec::new(); 9], |mut vec, line| {
                        for idx in (0..line.len()).step_by(4) {
                            let val = &line[idx+1..idx+2];
                            if val != " " { vec[idx/4].push(val) };
                        } 
                        vec
                    });

    moves.split("\n")
         .for_each(|w| {
            let mov: Vec<usize> = w.split(" ")
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect();
            
            let (num, from, to) = (mov[0], mov[1]-1, mov[2]-1);         
            
            for _i in 0..num {
                let val = stacks[from].pop().unwrap();
                stacks[to].push(val);
            }
         });

    for mut a in stacks {
        let val = a.pop().unwrap();
            print!("{}", val);
    }
    println!("");

}