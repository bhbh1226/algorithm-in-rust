use std::{io, collections::VecDeque};

fn do_bfs(check_coord: &mut [[bool; 50]; 50], real_coord: &[[bool; 50]; 50], m: u32, n: u32, queue: &mut VecDeque<(i32, i32)>) {
    while queue.is_empty() != true {
        let (x, y) = queue.pop_front().expect("pop error");

        if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 || check_coord[x as usize][y as usize] == true {
            continue;
        } 

        if real_coord[x as usize][y as usize] == true {
            check_coord[x as usize][y as usize] = true;

            queue.push_back((x - 1, y));
            queue.push_back((x + 1, y));
            queue.push_back((x, y - 1));
            queue.push_back((x, y + 1));
        }
    }
}

fn get_answer(check_coord: &mut [[bool; 50]; 50], real_coord: &[[bool; 50]; 50], m: u32, n: u32) -> u32 {
    let mut result = 0;
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    
    for i in 0..m as i32 {
        for j in 0..n as i32 {
            if check_coord[i as usize][j as usize] == true {
                continue;
            }

            check_coord[i as usize][j as usize] = true;

            if real_coord[i as usize][j as usize] == true {

                result += 1;
                // push in queue
                queue.push_back((i - 1, j));
                queue.push_back((i + 1, j));
                queue.push_back((i, j - 1));
                queue.push_back((i, j + 1));
                
                do_bfs(check_coord, real_coord, m, n, &mut queue)
            }
        }
    }

    result
}

fn main() {
    let mut t = String::new();

    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");

    let t:u32 = t
        .trim()
        .parse()
        .expect("this is not a number.");
    
    for _test_case in 0..t {
        let mut first_string = String::new();
        let mut check_coord = [[false; 50]; 50];
        let mut real_coord = [[false; 50]; 50];

        io::stdin()
            .read_line(&mut first_string)
            .expect("Failed to read line");

        let nums: Vec<u32> = first_string
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("THis is not a number."))
            .collect();
        
        let m = *nums.get(0).expect("This is not a valid number.");
        let n = *nums.get(1).expect("This is not a valid number.");
        let k = *nums.get(2).expect("This is not a valid number.");

        for _i in 0..k {
            let mut coord_string= String::new();

            io::stdin()
                .read_line(&mut coord_string)
                .expect("Failed to read line");

            let nums: Vec<u32> = coord_string
                .trim()
                .split(' ')
                .map(|x| x.parse().expect("THis is not a number."))
                .collect();

            let x = *nums.get(0).expect("This is not a valid number.");
            let y = *nums.get(1).expect("This is not a valid number.");    

            real_coord[x as usize][y as usize] = true;
        }

        let result = get_answer(&mut check_coord, &real_coord, m, n);
        
        println!("{result}");
    }
}
