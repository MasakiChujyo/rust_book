use rand::Rng;
use std::char::MAX;
use std::cmp::Ordering;
use std::io;

// VSCode + Rust shortcut
// Run: Ctrl+R Ctrl+R
// Save Ctrl+S

fn main() {
    ch3();
}

fn ch2() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret number is {}", secret_number);

    loop {
        println!("Pls input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Frain to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Big"),
            Ordering::Equal => {
                println!("Same");
                break;
            }
        }
    }
}

fn ch3() {
    let x = 2;
    let mut c = 0;
    const MAX_POINT: u32 = 100_000;
    loop {
        let x = 1;
        c += 1;
        if c > 10 {
            break;
        }
    }
    println!("{} {} {}", x, c, MAX_POINT);
}
