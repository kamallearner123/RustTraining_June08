#include <stdio.h>
#include <signal.h>
#include <unistd.h>

void signal_handler(int sig)
{
    printf("\nCaught signal %d (SIGINT)\n", sig);
}

int main()
{
    signal(SIGINT, signal_handler);

    while (1)
    {
        printf("Running...\n");
        sleep(1);
    }

    return 0;
}
