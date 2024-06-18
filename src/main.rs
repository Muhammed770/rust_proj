struct  User {
    name: String,
    age: u16,
    email: String
}
impl User {
    fn is_adult(&self) -> bool {
        if self.age>=18 {
            return true;
        }
        false
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index,character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}
fn main() {
    let email = String::from("mail.muhammed2002@gmail.com");
    let user = User {
        name:String::from("Muhammed"),
        age:16,
        email
    };
    println!("{} is {} year old : mail : {}",user.name,user.age, user.email);
    if user.is_adult() {
        println!("adult pass initiaited!!");
    } else {
        println!("access denied,not an adult");
    }
    let my_string = String::from("rrrrrrrrattatat");
    match find_first_a(my_string) {
        Some(index) => println!("found first a at {} ",index),
        None => print!("not found")
    }
}