use std::io;

fn main() {
    let mut first_string = String::new();
    let mut result = 0;
    let u32_2: u32 = 2;

    io::stdin()
        .read_line(&mut first_string)
        .expect("Failed to read line");

    let nums: Vec<u32> = first_string
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("THis is not a number."))
        .collect();
    
    let n = *nums.get(0).expect("This is not a valid number.");
    let r = *nums.get(1).expect("This is not a valid number.");
    let c = *nums.get(2).expect("This is not a valid number.");
    
    for i in 0..n {
        let r = r % (u32_2.pow(i + 1));
        let c = c % (u32_2.pow(i + 1));

        result += u32_2.pow(2 * i + 1) * (r / (u32_2.pow(i)));
        result += u32_2.pow(2 * i) * (c / (u32_2.pow(i)));
    }

    println!("{result}");
}
