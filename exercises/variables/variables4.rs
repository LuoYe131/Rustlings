// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut x = 3;//rust中变量默认声明为不可变immutable variable，要更改变量数据必须生命为mut
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
