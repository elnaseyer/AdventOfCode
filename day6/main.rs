

fn main() {
    fun(4);
    fun(14);
}

fn fun(unique: usize) {
    let (_chrs, i) = include_str!("input.txt")
                .chars()
                .enumerate()
                .fold((String::from(""), 0), |(mut chars,mut num), (i, ch)| {
                    if chars.len() < unique && num == 0{
                        if chars.contains(ch) {
                            let index = chars.find(ch).unwrap()+1;
                            chars = chars.chars().skip(index).collect();
                            chars.push(ch);
                        } else {
                            chars.push(ch);
                        } 
                        if chars.len() == unique {num = i};
                    }                      
                    (chars, num)
                });

    println!("'{}'", i+1);
}