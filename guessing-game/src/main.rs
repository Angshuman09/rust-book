use std::io;

fn main(){
    println!("Enter the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read lines");

    println!("Your guess: {}", guess);
}