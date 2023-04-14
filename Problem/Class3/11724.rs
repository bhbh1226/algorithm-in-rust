use std::io;

mod structure {
    use std::{collections::VecDeque, iter::repeat};

    const MAX_SIZE:usize = 1000;

    pub struct Graph {
        size: usize,
        check_list: [bool; MAX_SIZE],
        link_vec: Vec<Vec<usize>>,
    }

    impl Graph {
        pub fn new(size: usize) -> Graph {
            Graph { 
                size,
                check_list: [false; MAX_SIZE],
                link_vec: repeat(vec![]).take(size).collect(),
            }
        }

        pub fn add(&mut self, v: usize, w: usize) {
            let link = self.link_vec.get_mut(v).expect("Get element Failed.");
            link.push(w);
            link.sort();

            let link = self.link_vec.get_mut(w).expect("Get element Failed.");
            link.push(v);
            link.sort();
        }

        pub fn bfs(&mut self, from: usize) {
            let mut queue = VecDeque::<(u32, usize)>::new();

            queue.push_back((0, from));

            while queue.is_empty() == false {
                let (depth, number) = queue.pop_front().expect("Pop failed");

                if self.check_list[number] == true {
                    continue;
                }
                
                self.check_list[number] = true;

                for next_v in self.link_vec.get(number).expect("Get element Failed") {                    
                    if self.check_list[*next_v as usize] == false {
                        queue.push_back((depth + 1, *next_v));
                    }
                }
            }
        }

        pub fn get_result(&mut self) -> u32 {
            let mut result = 0;

            for i in 0..self.size {
                if self.check_list[i] == false {
                    self.bfs(i);
                    result += 1;
                }
            }

            result
        }
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

    let n = *nums.get(0).expect("This is not a valid number.");
    let m = *nums.get(1).expect("This is not a valid number.");
    let mut graph = structure::Graph::new(n);

    for _i in 0..m {
        let mut params_string = String::new();
    
        io::stdin().read_line(&mut params_string).expect("Failed to read line.");

        let nums: Vec<usize> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("This is not a number."))
            .collect();

        let v = *nums.get(0).expect("This is not a valid number.");
        let w = *nums.get(1).expect("This is not a valid number.");
            
        graph.add(v - 1, w - 1)
    }

    println!("{}", graph.get_result());

}