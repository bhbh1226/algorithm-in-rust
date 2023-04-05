use std::{io, collections::VecDeque, cmp::Ordering};

fn get_answer(n: u32) -> u32 {
    let mut queue = VecDeque::<(u32, u32)>::new(); // depth, number
    let mut result = 0;

    queue.push_back((0, n));
    
    while queue.is_empty() == false {
        let (depth, number) = queue.pop_front().expect("pop failed");

        match number.cmp(&1) {
            Ordering::Less => { continue; },
            Ordering::Equal => { result = depth; break; },
            Ordering::Greater => { 
                if number % 3 == 0 {
                    queue.push_back((depth + 1, number / 3));
                }
                if number % 2 == 0 {
                    queue.push_back((depth + 1, number / 2));
                }
                queue.push_back((depth + 1, number - 1));
            },
        }
    }

    result
}

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n:u32 = n.trim().parse().expect("this is not a number.");
    let result = get_answer(n);
    
    println!("{result}");
}
