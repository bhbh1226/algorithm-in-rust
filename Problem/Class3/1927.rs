use std::io;

mod structure {
    const MAX_SIZE: usize = 100001;

    pub struct MinHeap {
        size: usize,
        arr: [u32; MAX_SIZE],
    }

    impl MinHeap {
        pub fn new() -> MinHeap {
            MinHeap { 
                size: 0, 
                arr: [0; MAX_SIZE],
            }
        }

        pub fn insert(&mut self, x: u32) {
            let mut index = self.size + 1;

            while index != 1 && x < self.arr[index / 2] {
                self.arr[index] = self.arr[index / 2];
                index /= 2;
            }

            self.arr[index] = x;
            self.size += 1;
        }

        pub fn pop(&mut self) -> Option<u32> {
            if self.size == 0 { 
                return None 
            }
            
            let result = self.arr[1];

            self.arr[1] = self.arr[self.size];

            let mut parent = 1;
            
            loop {
                let mut child = parent * 2;

                if child + 1 <= self.size && self.arr[child] > self.arr[child + 1] {
                    child += 1;
                }

                if child > self.size || self.arr[child] > self.arr[parent] { 
                    break; 
                }

                // swap
                let swap_value = self.arr[parent];

                self.arr[parent] = self.arr[child];
                self.arr[child] = swap_value;

                parent = child;
            }

            self.size -= 1;
            Some(result)
        }
    }
}

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    let n:usize = n.trim().parse().expect("Failed to parse.");
    let mut heap = structure::MinHeap::new();

    for _i in 0..n {
        let mut x = String::new();

        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line.");

        let x:u32 = x.trim().parse().expect("Failed to parse.");

        if x == 0 {
            println!("{}", heap.pop().unwrap_or(0));
        } else {
            heap.insert(x);
        }

    }
}
