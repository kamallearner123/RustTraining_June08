#include <stdio.h>
#include <stdlib.h>
int main()
{
	char arr[10];
        // Issue: 1	
	if (arr[0] == 10)//Uninitialized
	{
		printf("Valid msg\n");
	}

	// Issue: 2
	char *ptr = malloc(100);
	//free(ptr);
	
	// Issue: 3
	unsigned char *ptr2 = malloc(10); // 0x1000 + 10 
	*(ptr2+10) = 100; // Invalid mem write
	free(ptr2);

	// Issue: 4
	int rank;
	if (rank == 1) {
		printf("1st Rank\n");
	}	

	// Issue: 5
	char *ptr3 = malloc(100);
	free(ptr3);
	ptr3[0] = 100;


	// Issue: 6
	char *ptr4 = malloc(100);
	free(ptr4);
	free(ptr4);
	
	

}
