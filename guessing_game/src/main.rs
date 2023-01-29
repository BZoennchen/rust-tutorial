use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let secret = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Input: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Output: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Equal => { println!("You win!"); break;},
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
    
}
