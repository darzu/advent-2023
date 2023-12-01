use std::fs;

pub fn day1() {
    println!("day1");

    let file_path = "./day1/input.txt";

    let input = fs::read_to_string(file_path).unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut first: u32 = 0;
        let mut seen_first = false;
        let mut last: u32 = 0;
        for c in line.chars() {
            if let Some(d) = c.to_digit(10) {
                if !seen_first {
                    seen_first = true;
                    first = d;
                }
                last = d;
            }
        }
        let val = first * 10 + last;
        sum += val;
    }
    println!("sum: {}", sum);
}
