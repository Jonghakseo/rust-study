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
        vector.push(vector[vector.len() - 2] +  vector[vector.len() - 1]);
    }

    vector
}



