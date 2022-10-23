use bcrypt::hash_with_salt;
use rand::Rng;
use std::{io, collections::HashMap};

fn main() {
    println!("Hello, world!");
}

struct User {
    username: String,
    password: String,
    salt: [u8; 16],
}

impl User {
    fn new(username: String, password: String) -> Self {
        let salt = generate_salt();
        Self {
            username,
            password: hash_password(password, salt),
            salt
        }
    }
}

fn hash_password(password: String, salt: [u8; 16]) -> String {
    hash_with_salt(password, 4, salt).unwrap().to_string()
}

fn create_user() -> User {

    let mut username = String::new();
    let mut stdin = io::stdin();

    println!("Enter username: ");
    stdin.read_line(&mut username);

    let mut password: String = String::new();
    stdin = io::stdin();

    println!("Enter Password: ");
    stdin.read_line(&mut password);

    User::new(username, password)
}

fn verify_login (user: User) -> bool {

    login(user);

    if (user_username == user.username) && (hash_password(user_password, user.salt) == user.password) {
        true
    } else { false }
}

fn login (users: HashMap<String, User>) -> User {
    let mut user_username = String::new();
    let mut stdin = io::stdin();

    println!("Enter your username: ");
    stdin.read_line(&mut user_username);

    let user = users.get(&user_username).unwrap();

    let mut user_password = String::new();
    stdin = io::stdin();

    println!("Enter your password: ");
    stdin.read_line(&mut user_password);
}

fn generate_salt() -> [u8; 16] {
    let mut salt: Vec<u8> = vec![];
    let mut rng = rand::thread_rng();
    let mut x = 0;

    while x < 16 {
        salt.push(rng.gen::<u8>());
        x += 1;
    }

    // generate random u8 in loop that repeats 16 times
    // push random num in loop (push_right)

    return salt.try_into().unwrap();
}

#[test]
fn can_generate_salt() {
    let salt = generate_salt();
    assert_eq!(salt.len(), 16);
}

#[test]
fn can_create_user() {
    let user = create_user("test".to_string(), "test".to_string());
    assert_eq!(user.username, "test");
    assert_eq!(user.password, hash_password("test".to_string(), user.salt));
}

#[test]
fn can_hash_password() {
    let salt = generate_salt();
    let password = "test".to_string();
    let hashed_password = hash_password(password, salt);
    assert_ne!(hashed_password, "test");
}

#[test]
fn can_login() {
    let user = create_user("test".to_string(), "test".to_string());
    assert_eq!(login(user), true);
}