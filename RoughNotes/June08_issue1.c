#include <stdio.h>
#include <string.h>
int main(int argc, char **argv) {
	int var2 = 20;
	char msg[10]; // array of chars of size 10
	int passwd_verdict = 0;

	// Copying the data coming from command / network
	strcpy(msg, argv[1]);
	printf("msg = %s\n", msg);

	// print neighbour values
	printf("passwd_verdict = %d\n", passwd_verdict);
	printf("var2 = %d\n", var2);
}	


