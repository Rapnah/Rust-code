use std::io;

fn main() 
{
    println!("1. Add Student");
    println!("2. Update Information");
    println!("3. Remove Student");
    println!("4. Search Student");
    println!("5. GPA Ranking");
    println!("6. Sort Name");
    println!("7. Student Sort");
    println!("8. Export File");
    let mut choose = String::new();
    io::stdin().read_line(&mut choose).expect("Error read line");
    //let choose: i32 = choose.trim().parse().expect("Please type a Number");
        match choose.as_str()
        {
            "1" => println!("choosing 1!!"),
            "2" => println!("choosing 2 @@"),
            &_ => println!("bt"),
        }
}

