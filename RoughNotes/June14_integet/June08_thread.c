#include <pthread.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

int shared_counter = 0;

void* increment(void* arg) {
    for (int i = 0; i < 100000; ++i) {
        shared_counter++; // Not atomic! Data race!
	sleep(1);
    }
    return NULL;
}

int main() {
    pthread_t t1, t2;
    pthread_create(&t1, NULL, increment, NULL);
    pthread_create(&t2, NULL, increment, NULL);
    pthread_join(t1, NULL); 
    pthread_join(t2, NULL);
    printf("Counter: %d\n", shared_counter); // 2,00,000 ? Random, 
    return 0;
}
