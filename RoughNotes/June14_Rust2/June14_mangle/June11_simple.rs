#[no_mangle]
pub extern "C" fn add_from_rust(a:i32, b:i32) ->i32 {
    return 0;
}

fn add_from_rust_2(a:i32, b:i32) ->i32 {
    return 0;
}

fn main() {
    println!("{}", add_from_rust(1,2));
}
