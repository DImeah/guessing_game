use std::io;

fn start_up() {
    let mut name = String::new();
    
    println!("Welcome to the Guesser !!!");
    println!("Please Enter your name: ");

    io::stdin()
        .read_line(&mut name).expect("Failed to read name");

}

fn guesser() {
    
}
fn main() {
    println!("Guess the number Game !");
    
    println!("Please input your Guess Number: ");
    
    let mut guess = String::new() ;

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    println!("Thank you for playing the game !");
    println!("Double of your guess is: {}", guess.trim().parse::<i32>().unwrap_or(0) * 2);
}
