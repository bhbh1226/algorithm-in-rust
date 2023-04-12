use std::io;

struct GreedyMeeting {
    meeting_vec: Vec<(u32, u32)>,
}

impl GreedyMeeting {
    pub fn new() -> GreedyMeeting {
        GreedyMeeting { 
            meeting_vec: Vec::<(u32, u32)>::new(), // start, end
        }
    }

    pub fn push_with_sort(&mut self, value: (u32, u32)) {
        let mut i:usize = 0;
        let (x, y) = value;

        while i < self.meeting_vec.len() {
            let &(tx, ty) = self.meeting_vec.get(i).expect("Failed to get value in vector");

            if ty < y {
                i += 1;
            } else if ty == y && tx < x {
                i += 1;
            } else {
                break;
            }
        }

        self.meeting_vec.insert(i, value);
    }

    pub fn get_result(&self) -> u32 {
        let mut count = 0;
        let mut end_time = std::u32::MIN;

        for i in 0..self.meeting_vec.len() {
            let &(x, y) = self.meeting_vec.get(i).unwrap();

            if x >= end_time {
                count += 1;
                end_time = y;
            }
        }

        count
    }
}

fn main() {
    let mut n = String::new();
    
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    
    let n: usize = n.trim().parse().expect("Failed to parse.");
    
    let mut meeting = GreedyMeeting::new();

    for _i in 0..n {
        let mut params_string = String::new();

        io::stdin().read_line(&mut params_string).expect("Failed to read line.");

        let nums: Vec<u32> = params_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("This is not a number."))
            .collect();

        let x = *nums.get(0).expect("This is not a valid number.");
        let y = *nums.get(1).expect("This is not a valid number.");

        meeting.push_with_sort((x, y));
    }

    // get result
    let result = meeting.get_result();

    println!("{result}");
}
