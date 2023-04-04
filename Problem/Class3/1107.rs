use std::{io, collections::VecDeque};

fn do_dfs(disable_list: &[bool; 10], first_diff: u32, n: u32) -> u32 {
    let max_channel = 500000;
    let mut min_value = max_channel;
    let mut queue = VecDeque::<(u32, u32)>::new();
    
    queue.push_back((0, 0));
    
    while queue.is_empty() != true {
        let (depth, number) = queue.pop_front().expect("pop failed");
        let cost = depth + number.abs_diff(n);

        if first_diff <= depth {
            break;
        }

        if cost <= min_value {
            if depth != 0 {
                min_value = cost;
            }

            // push in queue
            for i in 0..10 {
                let next_depth = depth + 1;
                let next_number = number * 10 + (i as u32);
                let next_cost = next_depth + next_number.abs_diff(n);

                if disable_list[i] == false && next_cost <= min_value {
                    queue.push_back((next_depth, next_number));
                }        
            }
        }
    }

    min_value = min_value.min(first_diff);

    min_value
}

fn get_answer(n: u32, disable_list: &[bool; 10]) -> u32 {
    let first_diff = n.abs_diff(100);

    do_dfs(disable_list, first_diff, n)
}

fn main() {
    let mut n = String::new();
    let mut m = String::new();
    let mut params_string = String::new();
    let mut disable_list = [false; 10];

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n:u32 = n.trim().parse().expect("This is not a number.");

    io::stdin()
        .read_line(&mut m)
        .expect("Failed to read line");

    let m:u32 = m.trim().parse().expect("This is not a number.");

    if m > 0 {
        io::stdin()
            .read_line(&mut params_string)
            .expect("Failed to read line");

        let nums: Vec<usize> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("THis is not a number."))
            .collect();
        
        for i in 0..m as usize {
            let index = *nums.get(i).expect("This is not a valid number.");

            disable_list[index] = true;
        }
    }

    let result = get_answer(n, &disable_list);
    
    println!("{result}");
}
