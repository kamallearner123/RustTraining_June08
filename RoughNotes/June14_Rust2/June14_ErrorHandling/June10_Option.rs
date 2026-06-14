
enum COLOR {
    RED,
    GREEN,
    BLUE
}


// enum <T> Option {
//      Some(T),
//      None
//  }
// enum <T1,T2> Result {
//      Ok(T1),
//      Err(T2)
// }
fn get_fine(val : u32) -> Option<i32> {
    if val > 90 {
        return Some(1000);
    } else {
        return None;
    }
}

fn main() {
    let a = get_fine(10);
    let b = a.unwrap(); // if Some(val), collect ortherwise panic!

    /*match a {
        Some(amount) => println!("Pay {} fine!!!", amount),
        None => println!("No fine")
    }*/
}

