#include <iostream>
int main() 
{
	int *ptr = new int[100];
	delete ptr;
	*ptr = 100;
}
