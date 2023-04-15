use std::io;

struct Paper {
    size: usize,
    paper: [[bool; 128]; 128],
}

impl Paper {
    pub fn new(size: usize) -> Paper {
        Paper {
            size,
            paper: [[false; 128]; 128],
        }
    }

    pub fn set_pos(&mut self, x: usize, y: usize, value: bool) {
        self.paper[x][y] = value;
    }

    fn get_after_paper(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> (u32, u32) {
        let mut result_white = 0;
        let mut result_blue = 0;
        let mut result_merge = false;

        if end_x - start_x == 2 {
            for x in start_x..end_x {
                for y in start_y..end_y {
                    if self.paper[x][y] == true {
                        result_blue += 1;
                    } else {
                        result_white += 1;
                    }
                }
            }
        } else {
            let next_size = (end_x - start_x) / 2;

            let (w1, b1) = self.get_after_paper(start_x, start_y, end_x - next_size, end_y - next_size);
            let (w2, b2) = self.get_after_paper(start_x + next_size, start_y, end_x, end_y - next_size);
            let (w3, b3) = self.get_after_paper(start_x, start_y + next_size, end_x - next_size, end_y);
            let (w4, b4) = self.get_after_paper(start_x + next_size, start_y + next_size, end_x, end_y);
        
            result_white = w1 + w2 + w3 + w4;
            result_blue = b1 + b2 + b3 + b4;
        }

        if (result_white == 4 && result_blue == 0) || (result_white == 0 && result_blue == 4) {
            result_merge = true;
        }

        if result_merge == true {
            (result_white / 4, result_blue / 4)
        } else {
            (result_white, result_blue)
        }
    }

    pub fn get_answer(&self) -> (u32, u32) {
        let (result_white, result_blue) = self.get_after_paper(0, 0, self.size, self.size);

        (result_white, result_blue)
    }
}

fn main() {
    let mut n = String::new();
    
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    
    let n: usize = n.trim().parse().expect("Failed to parse.");
    
    let mut paper = Paper::new(n);

    for i in 0..n {
        let mut params_string = String::new();

        io::stdin().read_line(&mut params_string).expect("Failed to read line.");

        let nums: Vec<u32> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("This is not a number."))
            .collect();

        for j in 0..n {
            let value = *nums.get(j).expect("This is not a valid number.");

            paper.set_pos(i, j, value == 1);
        }
    }

    let (result_white, result_blue) = paper.get_answer();
    
    println!("{result_white}");
    println!("{result_blue}");
}