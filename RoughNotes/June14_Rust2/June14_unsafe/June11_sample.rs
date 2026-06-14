fn main() {
    unsafe {
        let a = String::from("hello");
        let b = &a;
        a.push_str("...");
        println!(" b= {}", b);
    }
}
