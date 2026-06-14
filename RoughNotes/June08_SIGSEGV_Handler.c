#include <stdio.h>
#include <signal.h>
#include <stdlib.h>

void segv_handler(int sig)
{
    printf("Caught signal %d (SIGSEGV)\n", sig);
    exit(EXIT_FAILURE);
}

int main(void)
{
    signal(SIGSEGV, segv_handler);

    int *ptr = NULL;
    *ptr = 10;   // Generates SIGSEGV

    return 0;
}
