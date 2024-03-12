use core::cmp::Ordering;
use rand::Rng;
use std::io::{self, Write};
fn main() {
    println!("Guess the number!");
    println!("Please input your guess");
    let sec_num = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed");
        let guess: i32 = match guess.trim().parse() {
            Ok(gus) => gus,
            Err(_) => continue,
        };
        match guess.cmp(&sec_num) {
            Ordering::Less => print!("{} is Small, Try Higher!⬆️ : ", guess),
            Ordering::Greater => print!("{} is High, Try Smaller!⬇️ : ", guess),
            Ordering::Equal => {
                println!("Congrats, You Won!🙂");
                break;
            }
        }
        io::stdout().flush().unwrap();
    }
}
