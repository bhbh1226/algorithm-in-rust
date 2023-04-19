use std::io;

fn main() {
    let mut n = String::new();
    
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    
    let n: usize = n.trim().parse().expect("Failed to parse.");
    let mut parent_list = [0; 100001];
    parent_list[1] = 1;

    for _ in 1..n {
        let mut params_string = String::new();

        io::stdin().read_line(&mut params_string).expect("Failed to read line.");

        let nums: Vec<usize> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("This is not a number."))
            .collect();

        let node1 = *nums.get(0).expect("This is not a valid number.");
        let node2 = *nums.get(1).expect("This is not a valid number.");
    
        if parent_list[node1] == 0 {
            parent_list[node1] = node2;
        } else {
            parent_list[node2] = node1;
        }
    }

    for i in 2..n+1 {
        println!("{}", parent_list[i]);
    }
}