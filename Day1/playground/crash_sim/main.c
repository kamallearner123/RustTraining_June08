#include <stdio.h>

struct UserData {
    int id;
    char name[50];
};

void process_user(struct UserData* user) {
    printf("[*] Processing user data...\n");
    // Dereferencing a NULL pointer will cause a Segmentation Fault (SIGSEGV)
    printf("[+] User ID: %d\n", user->id);
    printf("[+] User Name: %s\n", user->name);
}

int main() {
    printf("--- Crash Simulator (Null Pointer Dereference) ---\n");
    
    struct UserData* current_user = NULL;
    
    printf("[*] Fetching user from database...\n");
    // Simulate a failure to fetch user, leaving pointer as NULL
    
    // Passing the NULL pointer to a function that expects valid memory
    process_user(current_user);
    
    return 0;
}
