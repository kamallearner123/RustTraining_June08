#include <stdio.h>

long long slow_function()
{
    long long sum = 0;

    for (long long i = 0; i < 100000000; i++)
    {
        sum += i;
    }

    return sum;
}

long long fast_function()
{
    long long sum = 0;

    for (long long i = 0; i < 1000000; i++)
    {
        sum += i;
    }

    return sum;
}

int main()
{
    printf("Slow = %lld\n", slow_function());
    printf("Fast = %lld\n", fast_function());

    return 0;
}
