use std::os::unix::fs::PermissionsExt;

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
fn main() {
    let email = String::from("mail.muhammed2002@gmail.com");
    let user = User {
        name:String::from("Muhammed"),
        age:16,
        email
    };
    println!("{} is {} year old",user.name,user.age);
    if user.is_adult() {
        println!("adult pass initiaited!!");
    } else {
        println!("access denied,not an adult");
    }
}