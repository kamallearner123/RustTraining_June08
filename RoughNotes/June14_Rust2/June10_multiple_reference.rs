fn main() {
    let mut data = String::from("world!!!");
    let mut ref1 = &data;
    let ref2 = &data;
    let ref3 = &data;

    // getting the complete control back to data
    data.push_str("...");

    // Checking ref1 is invalidated ?
    ref1 = &data;
    println!("ref1 = {}", ref1);

    println!("{:?}", data);
}
