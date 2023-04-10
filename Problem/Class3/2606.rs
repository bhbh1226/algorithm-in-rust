use std::io;

mod structure {
    use std::{collections::VecDeque, iter::repeat};

    const MAX_SIZE:usize = 100;

    pub struct Graph {
        _size: usize,
        check_list: [bool; MAX_SIZE],
        link_vec: Vec<Vec<u32>>,
    }

    impl Graph {
        pub fn new(_size: usize) -> Graph {
            Graph { 
                _size,
                check_list: [false; MAX_SIZE],
                link_vec: repeat(vec![]).take(_size).collect(),
            }
        }

        pub fn add(&mut self, v: u32, w: u32) {
            let link = self.link_vec.get_mut(v as usize).expect("Get element Failed.");
            link.push(w);
            link.sort();

            let link = self.link_vec.get_mut(w as usize).expect("Get element Failed.");
            link.push(v);
            link.sort();
        }

        pub fn bfs(&mut self, from: u32) {
            let mut queue = VecDeque::<(u32, u32)>::new();

            queue.push_back((0, from));

            while queue.is_empty() == false {
                let (depth, number) = queue.pop_front().expect("Pop failed");

                if self.check_list[number as usize] == true {
                    continue;
                }
                
                self.check_list[number as usize] = true;

                for next_v in self.link_vec.get(number as usize).expect("Get element Failed") {                    
                    if self.check_list[*next_v as usize] == false {
                        queue.push_back((depth + 1, *next_v));
                    }
                }
            }
        }

        pub fn get_checked_count(&self) -> u32 {
            let mut result = 0;

            for i in self.check_list {
                if i == true {
                    result += 1;
                }
            }

            result
        }
    }
}

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    let n:usize = n.trim().parse().expect("Failed to parse.");

    let mut m = String::new();

    io::stdin()
        .read_line(&mut m)
        .expect("Failed to read line.");

    let m:usize = m.trim().parse().expect("Failed to parse.");
    let mut graph = structure::Graph::new(n);

    for _i in 0..m {
        let mut params_string = String::new();

        io::stdin()
            .read_line(&mut params_string)
            .expect("Failed to read line.");

        let nums: Vec<u32> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("THis is not a number."))
            .collect();

        let n = *nums.get(0).expect("This is not a valid number.");
        let m = *nums.get(1).expect("This is not a valid number.");

        graph.add(n - 1, m - 1);
    }

    graph.bfs(0);

    println!("{}", graph.get_checked_count() - 1);
}
