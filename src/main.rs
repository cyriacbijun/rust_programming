use std::io;

fn main() {
    let mut guess = String::new();
    println!("Hello, enter your guess");
    io::stdin().read_line(&mut guess).expect("Line not read");
    println!("Here is your guess: {}",guess);

}
