// 使用
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing game start");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("secret_number is {}", secret_number);
    loop {
        //这个字符串是可变的
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // 强制类型转换
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("you guess number is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
