use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    //Reference count
    let data = Rc::new(RefCell::new(String::from("Rust Programming")));
    //let data = String::from("Rust");
    //let a = &mut data;
    let a = Rc::clone(&data); 
    let b = Rc::clone(&data);
    
    // Write: borroow_mut
    // Read: borrow
    a.borrow_mut().push_str(" is safe for critical systems!!!");

    println!("{}", b.borrow());
}
    
    
