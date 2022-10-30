extern crate guessing_number;
extern crate hello_cargo;
extern crate fibonacci;

fn main() {
    // for index in 1..17 {
    //     println!("{}", fibonacci::get_fibonacci_number(index));
    // }
    // for index in 1..17 {
    //     println!("{:?}", fibonacci::get_fibonacci_vector(index));
    // }
    println!("{}", fibonacci::get_fibonacci_struct(17));
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
    //
    //
    // hello_cargo::hello();
    // guessing_number::guess();
}
