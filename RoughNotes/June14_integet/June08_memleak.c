#include <stdlib.h>

void handleRequest() {
    int* sessionData = (int*)malloc(1024 * sizeof(int)); // 4KB allocated
}

int main() {
    handleRequest();
    return 0;
}
