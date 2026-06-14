/*
#include <stdio.h>
#include <stdlib.h>

int* fun() {
    int *ptr;
    if (0) {
        ptr = malloc(100);
    }
    return ptr;
}


int main() {
    int *handle = fun();
    if (handle == NULL) { // OK ??? 
        printf("Creating handler FAILED!!!");
    } else {
        printf("%d", *handle);
    }
}

*/


/*
 * class enum COLOR {
 *   RED = 0,
 *   GREEN = 10
 * };
*/

/*
 * Option<i32> 
 * Result<>
*/


#[derive(Debug, PartialEq)]
enum COLOR {
	RED,
	GREEN,
	BLACK,
        ERROR(String)
}



fn get_color(val:u8) -> COLOR {
    let ret_val:COLOR;
    if val < 100 {
        ret_val = COLOR::RED;
    } else if val <200 {
        ret_val =  COLOR::GREEN;
    } else if val <250{
        ret_val =  COLOR::BLACK;
    } else {
        //println!("Exceptional!!!");
        ret_val = COLOR::ERROR(String::from("invalid rang"));
    }
    return ret_val;
}


fn get_num () -> u8 {
    return 100;
}


fn main() {
    let color = get_color(255);

    // switch color {
    // case 1: break;
    // case 2: break;
    // default ---> Mandatory ?
    // }

    match color {
        COLOR::RED => {println!("RED is the color!!!")}        
        COLOR::ERROR(msg) => {println!("....ERROR!!! {}", msg)},
        _ => {println!("....")}
    }


    let num = get_num();
    match num {
        0..=10 => {println!("Value is between 0 to 10")},
        _ => {println!("Value is between 0 to 10")},
    }
}




