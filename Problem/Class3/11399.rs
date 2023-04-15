use std::{io, collections::BinaryHeap};

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    let n:usize = n.trim().parse().expect("Failed to parse.");
    let mut params_string = String::new();
    let mut heap = BinaryHeap::<u32>::new();

    io::stdin().read_line(&mut params_string).expect("Failed to read line.");

    let nums: Vec<u32> = params_string
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("This is not a number."))
        .collect();

    for i in 0..n {
        let value = *nums.get(i).expect("This is not a valid number.");

        heap.push(value);
    }

    let mut count = 1;
    let mut result = 0;

    while heap.is_empty() == false {
        result += heap.pop().expect("Failed to pop.") * count;
        count += 1;
    }

    println!("{result}");
}