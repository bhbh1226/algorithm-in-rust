use std::{io, collections::HashMap};

fn main() {
    let mut params_string = String::new();

    io::stdin()
        .read_line(&mut params_string)
        .expect("Failed to read line");

    let nums: Vec<usize> = params_string
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("This is not a number."))
        .collect();

    let n = *nums.get(0).expect("This is not a valid number.");
    let m = *nums.get(1).expect("This is not a valid number.");

    let mut n_idx2str_map = HashMap::<u32, String>::new();
    let mut n_str2idx_map = HashMap::<String, u32>::new();

    for i in 0..n {
        let mut params_string = String::new();

        io::stdin()
            .read_line(&mut params_string)
            .expect("Failed to read line");
        
        n_idx2str_map.insert((i + 1) as u32, params_string.trim().to_string().clone());
        n_str2idx_map.insert(params_string.trim().to_string().clone(), (i + 1) as u32);
    }

    for _i in 0..m {
        let mut params_string = String::new();

        io::stdin()
            .read_line(&mut params_string)
            .expect("Failed to read line");
        
        let params_string = params_string.trim();

        if params_string.parse::<u32>().is_ok() {
            println!("{}", n_idx2str_map.get(&params_string.parse::<u32>().unwrap()).unwrap());
        } else {
            println!("{}", n_str2idx_map.get(params_string).unwrap());
        }
    }
}
