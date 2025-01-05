use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game start");
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("secret_number is {}", secret_number);

    loop {
        let mut guess = String::new();
        println!("Please input your guess number");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        if compare_guess(guess, secret_number) {
            break;
        }
    }
}

// 比较输入和随机数
fn compare_guess(guess: u32, secret_number: u32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Your guess number {} is Too small", guess);
            false
        }
        Ordering::Greater => {
            println!("Your guess number {} is Too big", guess);
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}
