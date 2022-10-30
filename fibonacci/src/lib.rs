extern crate core;

use core::fmt;

pub fn get_fibonacci_number(fibonacci_index: u32) -> u32 {
    let mut p_2 = 0;
    let mut p_1 = 1;
    let mut now = 1;

    for _ in 1..fibonacci_index {
        now = p_2 + p_1;
        p_2 = p_1;
        p_1 = now;
    }

    now
}


pub fn get_fibonacci_vector(fibonacci_index: u32) -> Vec<u32> {
    let mut vector = Vec::new();
    vector.push(0);
    vector.push(1);

    for _ in 1..fibonacci_index {
        vector.push(vector[vector.len() - 2] + vector[vector.len() - 1]);
    }

    vector
}

pub struct Fibonacci {
    prev_1: u32,
    current: u32,
}

// fmt::Display 구현 -> 원하는 형태로 출력
impl fmt::Display for Fibonacci {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "previous: {}, current:{}", self.prev_1, self.current)
    }
}

pub fn get_fibonacci_struct(fibonacci_index: u32) -> Fibonacci {
    let mut fibonacci = Fibonacci {
        prev_1: 1,
        current: 0,
    };


    for _ in 1..fibonacci_index {
        fibonacci = Fibonacci {
            prev_1: fibonacci.current,
            current: fibonacci.prev_1 + fibonacci.current,
        };
    }

    fibonacci
}


