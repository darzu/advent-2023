use std::fs;

static NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn day1() {
    println!("day1");

    let file_path = "./day1/input.txt";

    let input = fs::read_to_string(file_path).unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut first: u32 = 0;
        let mut seen_first = false;
        let mut last: u32 = 0;

        for (i, c) in line.chars().enumerate() {
            let mut d: Option<u32> = None;
            // is it a digit?
            if let Some(d2) = c.to_digit(10) {
                d = Some(d2);
            }
            // is it a spelled digit?
            else if c.is_ascii_alphabetic() {
                for (ni, &num) in NUMS.iter().enumerate() {
                    if i + 1 >= num.len() {
                        let as_num = &line[i + 1 - num.len()..i + 1];
                        if num == as_num {
                            let ni_: u32 = ni.try_into().unwrap();
                            d = Some(ni_ + 1);
                        }
                    }
                }
            }
            // track it
            if let Some(d2) = d {
                if !seen_first {
                    seen_first = true;
                    first = d2;
                }
                last = d2;
            }
        }
        // sum it
        let val = first * 10 + last;
        sum += val;
    }
    println!("sum: {}", sum);
}
