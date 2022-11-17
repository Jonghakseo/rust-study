use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("검색을 위해서 인자를 2개 이상 입력해야 합니다. (검색하려는 문자열, 텍스트 파일)");
    }

    let query = &args[1];
    let filename = &args[2];

    println!("검색하려는 키워드: {}", query);
    println!("대상 파일: {}", filename);

    let mut file = File::open(filename).expect(&*format!("{} 파일을 찾을 수 없습니다.", filename));
    let mut file_text = String::new();

    file.read_to_string(&mut file_text).expect("읽을 수 없는 파일입니다.");

    println!("{}", file_text);
}
