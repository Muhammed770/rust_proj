fn main() {
    let greetings = String::from("hello world");
    for i in 0..greetings.len()+10 {
        let char1 = greetings.chars().nth(i);
        match char1 {
            Some(c) => print!("{} ", c),
            None => {
                println!();
                println!("no char found at {} :", i);
                break;
            }
        }
    }
    println!();
    println!("{}", greetings);
}
