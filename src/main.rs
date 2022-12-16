// Useful tool commands
// cargo fix
// cargo fmt
// cargo clippy

//use ferris_says::say;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin; //{stdin,stdout, BufWriter};

fn main() {
    let init_message = String::from("=====Guess the number!=====");
    /*     let width = init_message.len();
    let mut writer = BufWriter::new(stdout());
    say(init_message, width, &mut writer).unwrap();*/
    println!("{}", init_message);

    let secret_number = thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}\n");

    loop {
        println!("\nPlease input your guess");
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        for _i in 0..2 {
            // underscore to signify unused variable
        }
    }
}
