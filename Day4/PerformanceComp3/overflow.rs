fn fun(a:u8, b:u8) -> u8 {
    return a+b;
}


fn main() {
    let num1:u8 = 255;
    let num2:u8 = 255;

    let result = num1 + fun(1,2); //overalap, kepp max value 
    println!("result = {}", result);
}
    
