#include<stdio.h>

void fun(const int *ptr){
	int *ptr2 =(int*)ptr;
	*ptr2 += 1;
}

int main() {
	const int a= 10;
	const int *ptr = &a;
	fun(ptr);
	printf("a = %d\n", *ptr);
}
