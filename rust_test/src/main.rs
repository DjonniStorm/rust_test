use std::io;
fn main() {
    println!("Hello, Cargo!");
    let mut counter: u32 = 0; 
    loop {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("abc");
        println!("Hello, {}", input);
        counter += 1;
        if (counter >= 10) {
            break;
        }
    }
}