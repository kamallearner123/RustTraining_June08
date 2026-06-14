/*
int* get_handler() {
}

//-----
int main() {
    init();
    // strtok, strdup //
    int *h = get_hnalder();
    free(h);
    deinit();
}
#endif

get_hanlder
main
*/




// int fun(int, int) {
// }
// fn function_name (arg_name : data_type) -> return_data_type
// {
// }

fn print(msg:&mut String) {
    msg.push_str("...");
    println!("Message : Hello.. {}", msg);
}

fn print_n_times(num:i32) {
    println!("num = {}", num);
}


fn main() {
    let a = 10;
    let mut s1 = String::from("world!!!");
    // macro //
    println!("Before calling fun {}", s1);
    print(&mut s1); // Passing the owenership to "print" function
    println!("After calling fun {}", s1);
    print_n_times(a);
    println!("After calling {}", a);
}

