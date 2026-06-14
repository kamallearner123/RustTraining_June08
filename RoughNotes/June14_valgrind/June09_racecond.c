#include <pthread.h>
#include <stdio.h>

int shared_counter = 0;

void* increment(void* arg) {
    for (int i = 0; i < 10000; ++i) {
        // BUG: Data race! No mutex protecting this shared variable.
        shared_counter++;
    }
    return NULL;
}

int main() {
    pthread_t t1, t2;
    pthread_create(&t1, NULL, increment, NULL);
    pthread_create(&t2, NULL, increment, NULL);
    
    pthread_join(t1, NULL);
    pthread_join(t2, NULL);
    
    printf("Counter: %d\n", shared_counter);
    return 0;
}
