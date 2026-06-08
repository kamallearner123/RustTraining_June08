#include <stdlib.h>

int main() {
    int* ptr = (int*)malloc(sizeof(int));
    *ptr = 10;
    for (int i = 0; i < 100; ++i) {
        if (i == 50) {
            ptr = NULL; // Pointer suddenly becomes null
        }
        if (i == 75) {
            *ptr = 20; // Segfault here!
        }
    }
    return 0;
}
// In GDB:
// (gdb) run (crashes at line 10)
// (gdb) record
// (gdb) reverse-continue (finds exactly when ptr became null)
