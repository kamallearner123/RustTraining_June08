#include <stdio.h>

void func1()
{
    long long sum = 0;
    for(long long i = 0; i < 10000000; i++)
        sum += i;
}

void func2()
{
    long long sum = 0;
    for(long long i = 0; i < 50000000; i++)
        sum += i;
}

void func3()
{
    long long sum = 0;
    for(long long i = 0; i < 20000000; i++)
        sum += i;
}

int main()
{
    func1();
    func2();
    func3();

    return 0;
}
