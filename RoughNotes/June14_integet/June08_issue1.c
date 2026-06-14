#include <stdio.h>
int reset_counter =0;
int main()
{
	unsigned char count = 1;
	for (int i = 0; i<1000; i++)
	{
		count += 1;
		if (count == 0) {
			reset_counter += 1;	
		}
	}
	printf("count = %d\n", count);
}
