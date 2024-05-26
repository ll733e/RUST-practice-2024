use std::io;

fn main() {
    let mut buf = String::new();
    let test = io::stdin().read_line(&mut buf).unwrap();

    let number: u64 = buf.trim()
                        .parse()
                        .expect("Invalid input : Number Only");
    
    buf.clear();

    if number % 2 == 1 {
        println!("{number} is ODD!");
    } else {
        println!("{number} is EVEN!");
    }
}
