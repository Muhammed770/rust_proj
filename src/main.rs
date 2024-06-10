fn main() {
    let greetings = String::from("hello world");
    let first_word = get_first_word(&greetings);
    // for i in 0..greetings.len()+10 {
    //     let char1 = greetings.chars().nth(i);
    //     match char1 {
    //         Some(c) => print!("{} ", c),
    //         None => {
    //             println!();
    //             println!("no char found at {} :", i);
    //             break;
    //         }
    //     }
    // }
    println!();
    println!("{}",greetings);
    println!("the first word is :{}", first_word);
}

fn get_first_word(sentence:&str) -> String {
    let mut ans = String::new();
    for c in sentence.chars() {
        if c.is_whitespace() {
            break;
        }
        ans.push(c)
    }
    return ans;
}