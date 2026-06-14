#include <stdlib.h>

int fun() {
	char *addr = malloc(10);
	free(addr);
	free(addr);
}

int main() {
	fun();
}
