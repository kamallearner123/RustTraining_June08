#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define SIZE 4096

int main()
{
    static int matrix[SIZE][SIZE];

    clock_t start = clock();

    for(int j = 0; j < SIZE; j++) // go to each row 
    {
        for(int i = 0; i < SIZE; i++) // go to each column
        {
            matrix[i][j]++;
        }
    }

    clock_t end = clock();

    printf("Time = %.3f sec\n",
           (double)(end - start) / CLOCKS_PER_SEC);

    return 0;
}
