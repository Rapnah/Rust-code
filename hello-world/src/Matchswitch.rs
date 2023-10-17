use std::io;

fn main() 
{
    let mut choose = String::new();

    io::stdin().read_line(&mut choose).expect("Error read line");
    let choose: u32 = choose.trim().parse().expect("Please type a Number");
    println!("{}",choose);

    match choose{
        3 => {println!("yay")}
        4 => {println!("mooo")}
    }
}