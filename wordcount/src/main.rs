use std::collections::HashMap;

fn main() {
    let text = "피자 햄버거 햄버거 햄버거 피자 피자 햄버거 피자 피자 피자 피자 피자 햄버거 햄버거 햄버거 피자 피자";

    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
    
}
