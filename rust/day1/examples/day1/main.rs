use std::env;
use std::fs;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    let filename = "day1.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let depths: Vec<i32> = contents
        .split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let q1_result = depths
        .windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum::<i32>();

    println!("Question 1: {}", q1_result);

    let q2_result = depths
        .windows(3)
        .map(|window| window.iter().sum::<i32>())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum::<i32>();
    println!("Question 1: {}", q2_result);
    let end = SystemTime::now();
    println!("Duration: {}", end.duration_since(start).unwrap().as_millis())
}
