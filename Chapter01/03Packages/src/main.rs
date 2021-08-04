use our_package::encryptor::{rot13, Encryptable};
use std::io;

fn main() {
    println!("Input the string you want to encrypt:");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");

    println!(
        "Your encrypted string: {}",
        rot13::Rot13(user_input).encrypt()
    );
}
