#include <stdio.h>
#include <stdlib.h>

int main() {
	char *ptr1 = malloc(100); // if the funciton declarations is not avaialble : Default return is "int"
	*ptr1 = 100;

	int *ptr2 = (int*)ptr1;
	free(ptr1);
}
