#include <stdio.h>
#include "my_header.h"
int gb = 100; // Initialized - Data Segment
int gcount; // Un-initialized: Default value: 0: BSS  

int fun1(int a, int b) {
	//int a; // Stack  Segment: Defult: Garbage
	//int b; // Stack  Segment: D.....
	int c = a+b;
	return c;
}

