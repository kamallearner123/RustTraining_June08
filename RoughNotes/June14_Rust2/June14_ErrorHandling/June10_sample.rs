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
        NONE
}



fn get_color(val:u8) -> COLOR {
    let ret_val:COLOR;
    if val < 100 {
        ret_val = COLOR::RED;
    } else if val <200 {
        ret_val =  COLOR::GREEN;
    } else if val <250{
        ret_val =  COLOR::BLACK;//COLOR::BLACK;
    } else {
        //println!("Exceptional!!!");
        ret_val =  100; // COLOR::NONE;
    }
    return ret_val;

}


fn main() {
    let color = get_color(100);
    if color==COLOR::NONE
    {
        println!("ERRORR.... ");
    } else{
        println!("color = {:?}", color);
    }
}




