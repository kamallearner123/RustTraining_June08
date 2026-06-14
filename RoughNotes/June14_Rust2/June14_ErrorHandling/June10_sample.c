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
 * Option<i32> 
 * Result<>
*/



enum COLOR {
	RED,
	GREEN,
	BLACK
}

fn get_color(val:u8) -> COLOR {
}



