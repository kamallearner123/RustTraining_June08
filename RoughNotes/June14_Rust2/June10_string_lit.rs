fn main() {
    let s = "Hello"; //String::fromi("Hello"); // array of chars //
    let t = s; // Copies the data of s to t //
    
    let a = [String::from("Hello"), String::from("World")];
    let b = a;


    println!("s = {}", s);
    println!("t = {}", t);

    println!("a = {:?}", a);
}

