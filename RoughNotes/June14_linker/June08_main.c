#include <stdio.h>
#include "my_header.h"
extern int gb; // Global variable: Data segment
int main()
{
	char *statement = "India is great!!!"; //READ-ONLY memory
	int result = fun1(10, 20);
	printf("gb = %d\n", gb);
	printf("result = %d\n", result);
}
