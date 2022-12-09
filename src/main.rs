//use ferris_says::say; // from the previous step
use std::io::stdin;//{stdin,stdout, BufWriter};
use rand::{thread_rng,Rng};
use std::cmp::Ordering;

fn main() {
/*
    let stdout = stdout();

    let init_message = String::from("   Guess the number!\nPlease input your guess.");
    let width = init_message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(init_message.as_bytes(), width, &mut writer).unwrap();
*/
    println!("\n   Guess the number!\nPlease input your guess");

    let secret_number = thread_rng()
        .gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

/*
    let message = String::from("You guessed: {guess}");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
*/

    println!("\nYou guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    for _i in 0..2 {
        // underscore to signify unused variable
        
    }
}
