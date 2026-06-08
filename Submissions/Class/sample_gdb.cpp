#include <iostream>

int main() {
    int critical_value = 0;
    for (int i = 0; i < 10000; ++i) {
        if (i == 9999) {
            critical_value = 1; // BUG triggers here
        }
    }
    std::cout << critical_value << std::endl;
    return 0;
}
// In GDB:
// (gdb) break 6 if i == 9999
// OR
// (gdb) watch critical_value
