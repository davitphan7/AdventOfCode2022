
use std::io;
fn main(){

    println!("INPUT GUESS NUMBER:");
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("PANIC");

    println!("your guess is: {guess}")
}