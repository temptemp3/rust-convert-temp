fn conv_f_to_c(f: i32) -> i32 {
   ( f - 32) * 5 / 9
}
fn conv_c_to_f(c: i32) -> i32 {
   c * 9 / 5 + 32
}
fn main() {
    println!("{}f", 10);
    println!("is {}c", conv_f_to_c(10));
    println!("{}c", 10);
    println!("is {}f", conv_c_to_f(10));
    println!("{}c", -40);
    println!("is {}f", conv_c_to_f(-40));
    println!("Hello, world!");
}
