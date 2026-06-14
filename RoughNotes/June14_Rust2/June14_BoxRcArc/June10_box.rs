fn main ()
{
    let mut ptr:Box<u8> = Box::new(10);
    println!("ptr is having {}", ptr);
    //let ptr2 = ptr;
    *ptr = 100;
    println!("ptr is having {}", ptr);
}

