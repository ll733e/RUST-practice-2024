use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let result = buf.to_lowercase()
                    .replace("a", "4")
                    .replace("b", "8")
                    .replace("e", "3")
                    .replace("g", "9")
                    .replace("i", "1")
                    .replace("l", "1")
                    .replace("o", "0")
                    .replace("r", "2")
                    .replace("s", "5")
                    .replace("t", "7");

        
    print!("{result}");
}
// very simple leet translator