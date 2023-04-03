use std::{io, vec, collections::HashMap};

fn fibo(fibo_map: &mut HashMap<u32, (u32, u32)>, n: u32) -> (u32, u32) {
    let result = match n {
        0 => (1, 0),
        1 => (0, 1),
        x => {
            match fibo_map.get(&n) {
                None => {
                    let fibo1 = fibo(fibo_map, x - 1);
                    let fibo2 = fibo(fibo_map, x - 2);
    
                    Some((fibo1.0 + fibo2.0, fibo1.1 + fibo2.1))
                },
                x => x.copied()
            }.expect("result is not valid")
        }
    };

    fibo_map.insert(n, result);

    result
}

fn main() {
    let mut t = String::new();
    let mut result_list: Vec<(u32, u32)> = vec![];
    let mut fibo_map = HashMap::new(); // for memoization

    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");

    let t:u32 = t
        .trim()
        .parse()
        .expect("This is not a number.");

    for _number in 0..t {
        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n:u32 = n
            .trim()
            .parse()
            .expect("This is not a number.");

        result_list.push(fibo(&mut fibo_map, n));
    }

    for element in result_list {
        let (zero, one) = element;
        println!("{zero} {one}");
    }

}