#include <stdio.h>
int add(int a, int b){
	int c = a+b;
	return c;
}

int main() {
	int result = add(10,20);
	printf("result = %d\n", result);
}
