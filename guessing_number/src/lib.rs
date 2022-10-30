extern crate rand;

use std::io::stdin;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};

pub fn guess() {
    let secret_num = thread_rng().gen_range(1..101);

    println!("Secret number is {}", secret_num);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // 개행 제거, 변수 쉐도잉
        let guess = guess.trim();

        // parse 자체가 제공되는 타입에 따라 다른 동작을 하게 됨
        // let guess:i32 = guess.parse().expect("parsing error");
        // or
        // let guess = guess.parse::<i32>().expect("parsing error");

        // 에러 처리 match로 가능
        let guess = match guess.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}
