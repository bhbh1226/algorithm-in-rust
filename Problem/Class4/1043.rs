use std::{io, iter::repeat};

mod structure {
    use std::iter::repeat;

    const MAX_SIZE:usize = 50;

    pub struct Graph {
        _size: usize,
        pub know_list: [bool; MAX_SIZE],
        check_list: [bool; MAX_SIZE],
        link_vec: Vec<Vec<u32>>,
    }

    impl Graph {
        pub fn new(_size: usize) -> Graph {
            Graph { 
                _size,
                know_list: [false; MAX_SIZE],
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
            link.dedup();

            let link = self.link_vec.get_mut(w as usize).expect("Get element Failed.");
            link.push(v);
            link.sort();
            link.dedup();
        }

        pub fn dfs(&mut self, from: usize) {
            if self.check_list[from] == true {
                return;
            }
            
            self.check_list[from] = true;
            self.know_list[from] = true;

            for next_v in self.link_vec.get(from).expect("Get element Failed").clone() {
                if self.check_list[next_v as usize] == false {
                    self.dfs(next_v as usize);
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
        .map(|x| x.parse().expect("This is not a number."))
        .collect();

    let n = *nums.get(0).expect("This is not a valid number.");
    let m = *nums.get(1).expect("This is not a valid number.");

    let mut graph = structure::Graph::new(n);

    let mut params_string = String::new();

    io::stdin()
        .read_line(&mut params_string)
        .expect("Failed to read line");

    let nums: Vec<u32> = params_string
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("This is not a number."))
        .collect();

    // 진실을 아는 사람
    let t_size = *nums.get(0).expect("This is not a valid number.");
    let mut t_vec = Vec::<u32>::new();

    for i in 0..t_size {
        t_vec.push(*nums.get((i + 1) as usize).expect("This is not a valid number.") - 1);
    }

    let mut p_vec: Vec<Vec<u32>> = repeat(vec![]).take(m as usize).collect();

    for i in 0..m {
        let mut params_string = String::new();

        io::stdin()
            .read_line(&mut params_string)
            .expect("Failed to read line");

        let nums: Vec<u32> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("This is not a number."))
            .collect();

        // 파티
        let p = *nums.get(0).expect("This is not a valid number.");

        for j in 0..p {
            let v_1 = *nums.get((j + 1) as usize).expect("This is not a valid number.") - 1;
            
            p_vec.get_mut(i as usize).expect("get failed.").push(v_1);

            if j < p - 1 {
                let v_2 = *nums.get((j + 2) as usize).expect("This is not a valid number.") - 1;
                
                graph.add(v_1, v_2);
            }
        }
    }

    for i in t_vec {
        graph.check_clear();
        graph.dfs(i as usize);
    }

    let mut result = 0;

    for i in p_vec {
        let mut is_know = false;

        for j in i {
            if graph.know_list[j as usize] == true {
                is_know = true;
                break;
            }
        }

        if is_know == false {
            result += 1;
        }
    }

    println!("{result}");
}
