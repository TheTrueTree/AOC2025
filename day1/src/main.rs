use std::fs::File;
use std::io::{BufRead, BufReader};

fn returnPosition(l: i32, r: i32, m: char) -> i32 {
    let x:i32 = if m == 'R' {l + r} else {if l - r % 100 < 0 {l - r % 100 + 100} else {l - r}};
    return x % 100;
}

fn main() {
    let reader = File::open("F:/Projects/AoC2025/day1/src/input.txt").expect("Should work.");
    let reader = BufReader::new(reader);

    let mut starting_position: i32 = 50;
    let mut password: u32 = 0;

    for line in reader.lines(){
        let command = line.expect("Bad practice unuseful message");
        let operator = command.chars().next().unwrap();
        
        let number = command.replace(operator, "");
        let number = number.parse::<i32>().unwrap();

        starting_position = returnPosition(starting_position, number as i32, operator);
        if starting_position == 0 {
            password = password + 1;
        }
    }

    println!("Password: {}", password)

}
