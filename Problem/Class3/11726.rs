use std::io;

struct DyWithMemo {
    memo: [u32; 1001],
}

impl DyWithMemo {
    pub fn new() -> DyWithMemo {
        DyWithMemo { 
            memo: [0; 1001], 
        }
    }

    pub fn get_answer(&mut self, n: usize) -> u32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        } else if self.memo[n] != 0 {
            return self.memo[n];
        } else {
            self.memo[n] = (self.get_answer(n - 2) + self.get_answer(n - 1)) % 10007;
            
            return self.memo[n];
        }
    }
}


fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    let n: usize = n.trim().parse().expect("Failed to parse.");
    let mut dynamic = DyWithMemo::new();

    println!("{}", dynamic.get_answer(n));
}