extern crate guessing_number;
extern crate hello_cargo;
extern crate rect;
extern crate fibonacci;

use rect::Rectangle;

fn main() {
    rect_exec();
    // let x = 5;
    //
    // let y = {
    //     let x = 3;
    //     // 표현식의 종결은 세미콜론(;)을 사용하지 않는다.
    //     // 세미콜론 사용시 평가 구문으로 인식되어 반환값이 아니게 됨;;
    //     x + 1
    // };
    //
    // println!("The value of y is: {}", y);
}

fn rect_exec() -> () {
    let big_rect = Rectangle { width: 20, height: 20 };
    let small_square = Rectangle::square(19);

    match big_rect.contain(&small_square) {
        true => println!("is Contain!"),
        false => println!("is Not Contain!")
    };
}

fn guessing_number_exec() {
    guessing_number::guess();
}

fn hello_cargo_exec() {
    hello_cargo::hello();
}

fn fibonacci_exec() {
    for index in 1..17 {
        println!("{}", fibonacci::get_fibonacci_number(index));
    }
    for index in 1..17 {
        println!("{:?}", fibonacci::get_fibonacci_vector(index));
    }
    println!("{}", fibonacci::get_fibonacci_struct(17));
}
