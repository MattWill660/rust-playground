use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    for _i in 0..2 {
        // underscore to signify unused variable
        say(message.as_bytes(), width, &mut writer).unwrap();
    }
}
