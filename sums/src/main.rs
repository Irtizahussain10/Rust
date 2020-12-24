use std::io;

fn main() {
    let mut number = String::new();
    println!("Enter the number");
    io::stdin().read_line(&mut number).expect("Invalid input");
    let number: u32 = number.trim().parse().expect("Invalid Input");
    let mut numbers = Vec::new();
    let mut i = 0;
    while i < number {
        i = i + 1;
        numbers.push(i);
    }
    let mut sum = 0;
    for i in &mut numbers {
        sum = sum + *i;
    }
    sum = sum.pow(2);
    println!("{}", sum);
    let mut sumsq: u32 = 0;
    for i in &mut numbers {
        sumsq = sumsq + i.pow(2);
    }
    println!("{}", sumsq);
}
