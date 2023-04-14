use std::{io, collections::VecDeque, iter::repeat};

struct TomatoBox {
    m: usize,
    n: usize,
    arr: Vec<Vec<i32>>,
}

impl TomatoBox {
    pub fn new(m: usize, n: usize) -> TomatoBox {
        TomatoBox { 
            m, 
            n, 
            arr: repeat(vec![]).take(n).collect(), 
        }
    }

    pub fn push_at(&mut self, x: usize, value: i32) {
        self.arr[x].push(value);
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

            if x > 0 {
                if self.arr[x - 1][y] == 0 {
                    queue.push_back((depth + 1, x - 1, y));
                }
            } if x + 1 < self.n {
                if self.arr[x + 1][y] == 0 {
                    queue.push_back((depth + 1, x + 1, y));
                }
            } if y > 0 {
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
        let mut is_trigger_exists = true;
        let mut result = -1; // is 1 exists로 바꾸기
        let mut queue = VecDeque::<(i32, usize, usize)>::new();

        for i in 0..self.n {
            for j in 0..self.m {
                if self.arr[i][j] == 1 {
                    is_trigger_exists = false;
                
                    queue.push_back((0, i, j));
                }
            }
        }

        if is_trigger_exists == false {
            result = self.do_bfs(&mut queue);
            
            for i in 0..self.n {
                for j in 0..self.m {
                    if self.arr[i][j] == 0 {
                        result = -1;
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
        
            tbox.push_at(i, value);
        }
    }

    println!("{}", tbox.get_answer());
}