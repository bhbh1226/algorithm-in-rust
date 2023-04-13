use std::{io, collections::VecDeque};

struct TomatoBox {
    m: usize,
    n: usize,
    arr: [[i32; 1000]; 1000],
}

impl TomatoBox {
    pub fn new(m: usize, n: usize) -> TomatoBox {
        TomatoBox { 
            m, 
            n, 
            arr: [[-1; 1000]; 1000], 
        }
    }

    pub fn set_pos(&mut self, x: usize, y:usize, value: i32) {
        self.arr[x][y] = value;
    }

    fn do_bfs(&mut self, queue: &mut VecDeque<(i32, usize, usize)>) -> i32 { // depth, x, y
        let mut result = 0;

        while queue.is_empty() == false {
            let (depth, x, y) = queue.pop_front().expect("Failed to pop.");

            if depth != 0 && self.arr[x][y] == 1 {
                continue;
            }

            result = result.max(depth);
            self.arr[x][y] = 1;

            if x - 1 > 0 {
                if self.arr[x - 1][y] == 0 {
                    queue.push_back((depth + 1, x - 1, y));
                }
            } if x + 1 < self.n {
                if self.arr[x + 1][y] == 0 {
                    queue.push_back((depth + 1, x + 1, y));
                }
            } if y - 1 > 0 {
                if self.arr[x][y - 1] == 0 {
                    queue.push_back((depth + 1, x, y - 1));
                }
            } if y + 1 < self.m {
                if self.arr[x][y + 1] == 0 {
                    queue.push_back((depth + 1, x, y + 1));
                }
            }
        }

        result
    }

    pub fn get_answer(&mut self) -> i32 {
        let mut result = 0;
        let mut queue = VecDeque::<(i32, usize, usize)>::new();

        for i in 0..self.n {
            for j in 0..self.m {
                if self.arr[i][j] == 1 {
                    result = -1;
                
                    queue.push_back((0, i, j));
                }
            }
        }

        if result == 0 {
            let cost = self.do_bfs(&mut queue);
            
            for i in 0..self.n {
                for j in 0..self.m {
                    if self.arr[i][j] == 0 {
                        result = cost;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let mut params_string = String::new();
    
    io::stdin().read_line(&mut params_string).expect("Failed to read line.");

    let nums: Vec<usize> = params_string
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("This is not a number."))
        .collect();

    let m = *nums.get(0).expect("This is not a valid number.");
    let n = *nums.get(1).expect("This is not a valid number.");

    let mut tbox = TomatoBox::new(m, n);

    for i in 0..n {
        let mut params_string = String::new();
        
        io::stdin().read_line(&mut params_string).expect("Failed to read line.");

        let nums: Vec<i32> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("This is not a number."))
            .collect();    
        
        for j in 0..m {
            let value = *nums.get(j).expect("This is not a valid number.");
        
            tbox.set_pos(i, j, value);
        }
    }

    println!("{}", tbox.get_answer());
}