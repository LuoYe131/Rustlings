// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {  //如何函数体中以表达式结尾且表达式没有分号，该表达式会自动被认为是函数的返回值，带分号则返回值被丢弃
    num * num
}
