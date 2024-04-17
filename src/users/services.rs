
use serde::Serialize;

#[derive(Serialize)] 
pub struct User {
    name: String,
}


pub fn users_list() -> Vec<User> {
    let users = vec![
        User { name: String::from("Alice") },
        User { name: String::from("Bob") },
    ];

    users
}