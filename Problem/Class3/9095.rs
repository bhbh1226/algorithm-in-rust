use std::io;

struct DyWithMemo {
    memo: [u32; 11],
}

impl DyWithMemo {
    pub fn new() -> DyWithMemo {
        DyWithMemo { 
            memo: [0; 11], 
        }
    }

    pub fn get_answer(&mut self, n: usize) -> u32 {
        
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        } else if n == 3 {
            return 4;
        } else if self.memo[n] != 0 {
            return self.memo[n];
        } else {
            self.memo[n] = self.get_answer(n - 1) + self.get_answer(n - 2) + self.get_answer(n - 3);
            
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
    
    for _ in 0..n {
        let mut m = String::new();
        
        io::stdin()
            .read_line(&mut m)
            .expect("Failed to read line.");

        let m: usize = m.trim().parse().expect("Failed to parse.");
        
        println!("{}", dynamic.get_answer(m));
    }
}