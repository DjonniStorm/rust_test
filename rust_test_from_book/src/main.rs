fn main() {
    println!("Hello, world!");
    let hello:String = String::from("Hello");
    let world:&String = &hello.clone();
    println!("{}", str(hello, world))
}
fn str (str : String, str2: &String) -> String {
    let arr: [String; 2] = [str2.clone(), str];
    return arr[0].clone();
}