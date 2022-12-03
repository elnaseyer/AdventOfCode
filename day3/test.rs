
fn main() {
    let name = "hej jag \n heter \n elna \n och \n jag \n vill få detta att funka";

    name.split(4*\n)
        .for_each(|l|     println!("'{l}' \n \n"));



    }
