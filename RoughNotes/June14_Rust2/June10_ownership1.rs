use std::mem;
fn main() {
    // int *ptr = new int; -- not safe
    // uniq_ptr<int> p1 = uniq_ptr{new int}; -- restrict/safe
    let s1:String = String::from("hello world!!!");
    let mut s2 = s1.clone(); //s2 = std::move(s1); // duplicating pointer //
    s2.push_str(" Good morning!!!");
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    let num1 = 100; // const int num1 =100; num1 =num1+2;
    let mut num2 = num1;
    num2 += 1;
    println!("num1 = {}", num1);
    println!("num2 = {}", num2);

    println!("Size of num1 = {}", mem::size_of_val(&num1));
}
