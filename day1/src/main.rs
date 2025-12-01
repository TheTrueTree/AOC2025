use std::fs::File;
use std::io::{BufRead, BufReader};

fn return_position(l: i32, r: i32, m: char) -> i32 {
    let x:i32 = if m == 'R' {l + r} else {if l - r < 0 {(l - r) % 100 + 100} else {l - r}};
    return x % 100;
}

fn return_overload(l: i32, r: i32, m: char) -> u32{
    let x:i32 = if m == 'R' {l + r} else {l - r};
    let overload:i32 = x.abs() / 100;
    return overload as u32;
}

fn main() {
    let reader = File::open("F:/Projects/AoC2025/AOC2025/day1/src/input.txt").expect("Should work.");
    let reader = BufReader::new(reader);

    let mut starting_position: i32 = 50;
    let mut password: u32 = 0;

    for line in reader.lines(){
        let mut overloads: u32 = 0;
        let command = line.expect("Bad practice unuseful message");
        let operator = command.chars().next().unwrap();
        
        let number = command.replace(operator, "");
        let number = number.parse::<i32>().unwrap();

        starting_position = return_position(starting_position, number as i32, operator);
        overloads = return_overload(starting_position, number as i32, operator);
    
        if starting_position == 0 {
            password = password + 1;
        } 
        
        password = password + overloads;

        println!("Start: {}, Command: {}, Overloads: {}, Password: {}", starting_position, command, overloads, password);
        
    }

    println!("Password: {}", password)

}
