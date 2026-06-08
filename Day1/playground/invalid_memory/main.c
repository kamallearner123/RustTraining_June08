#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void simulate_invalid_memory_access() {
    printf("[*] Allocating 10 bytes of memory on the heap...\n");
    char* buffer = (char*)malloc(10 * sizeof(char));
    
    if (buffer == NULL) {
        printf("[-] Memory allocation failed!\n");
        return;
    }

    printf("[*] Attempting to write 50 bytes into the 10-byte buffer...\n");
    // Invalid memory access: Heap Buffer Overflow
    strcpy(buffer, "This is a very long string that will definitely overflow the allocated 10 bytes!");
    
    printf("[+] String copied: %s\n", buffer);

    printf("[*] Attempting to free the corrupted buffer...\n");
    // This will likely crash or corrupt the heap manager's metadata
    free(buffer);
    printf("[+] Memory freed successfully (Wait, really?).\n");
}

int main() {
    printf("--- Invalid Memory Access Simulator ---\n");
    simulate_invalid_memory_access();
    return 0;
}
