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

    pub fn pop_front(&mut self) -> Option<(u32, u32)> {
        self.meeting_vec.pop()
    }

    pub fn push_with_sort(&mut self, value: (u32, u32)) {
        let mut i:usize = 0;
        let (_x, y) = value;

        while i < self.meeting_vec.len() {
            if self.meeting_vec.get(i).unwrap().1 > y {
                i += 1;
            } else {
                break;
            }
        }

        self.meeting_vec.insert(i, value);
    }

    pub fn is_empty(&self) -> bool {
        self.meeting_vec.is_empty()
    }

    pub fn exclude(&mut self, value: u32) {
        self.meeting_vec.retain(|a| a.0 >= value);
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
    let mut result = 0;

    while meeting.is_empty() != true {
        let (_x, y) = meeting.pop_front().expect("Failed to pop.");

        meeting.exclude(y);
        result += 1;
    }

    println!("{result}");
}
