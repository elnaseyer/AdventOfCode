use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open file.txt"));

    let mut vec: Vec<i32> = Vec::new();
    vec.push(0);

    for line in reader.lines() {
        for word in line.unwrap().split("\n") {
            if word == "" {
                vec.push(0);
            } else {
                let index = vec.len();
                vec[index-1] += word.parse::<i32>().unwrap();
            }
            
        }
    }

    vec.sort();

    let max = vec.pop().unwrap();
    let max3 = max + vec.pop().unwrap() + vec.pop().unwrap();
    
    println!("The maximum value is '{max}'");
    println!("The maximum 3 values are '{max3}'");

}