use std::io;

mod structure {
    use std::{collections::VecDeque, iter::repeat};

    const MAX_SIZE:usize = 1000;

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

        pub fn check_clear(&mut self) {
            self.check_list.fill(false);
        }

        pub fn add(&mut self, v: u32, w: u32) {
            let link = self.link_vec.get_mut(v as usize).expect("Get element Failed.");
            link.push(w);
            link.sort();

            let link = self.link_vec.get_mut(w as usize).expect("Get element Failed.");
            link.push(v);
            link.sort();
        }

        pub fn dfs(&mut self, from: usize) {
            if self.check_list[from] == true {
                return;
            }
            
            self.check_list[from] = true;
            print!("{} ", from + 1);

            for next_v in self.link_vec.get(from).expect("Get element Failed").clone() {
                if self.check_list[next_v as usize] == false {
                    self.dfs(next_v as usize);
                }
            }
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
                print!("{} ", number + 1);

                for next_v in self.link_vec.get(number as usize).expect("Get element Failed") {                    
                    if self.check_list[*next_v as usize] == false {
                        queue.push_back((depth + 1, *next_v));
                    }
                }
            }
        }
    }
}

fn main() {
    let mut params_string = String::new();

    io::stdin()
        .read_line(&mut params_string)
        .expect("Failed to read line");

    let nums: Vec<usize> = params_string
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("THis is not a number."))
        .collect();

    let n = *nums.get(0).expect("This is not a valid number.");
    let m = *nums.get(1).expect("This is not a valid number.");
    let v = *nums.get(2).expect("This is not a valid number.");

    let mut graph = structure::Graph::new(n);

    for _i in 0..m {
        let mut params_string = String::new();

        io::stdin()
            .read_line(&mut params_string)
            .expect("Failed to read line");

        let nums: Vec<u32> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("THis is not a number."))
            .collect();

        let v_1 = *nums.get(0).expect("This is not a valid number.") - 1;
        let v_2 = *nums.get(1).expect("This is not a valid number.") - 1;

        graph.add(v_1, v_2);
    }

    graph.check_clear();
    graph.dfs(v - 1);
    println!();
    graph.check_clear();
    graph.bfs((v - 1) as u32);
}
