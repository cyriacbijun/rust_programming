use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Hello, enter your guess");
    io::stdin().read_line(&mut guess).expect("Line not read");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    println!("Here is your guess: {} and the actual number : {} ",guess,secret_number);

}
