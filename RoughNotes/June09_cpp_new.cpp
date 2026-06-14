#include <iostream>
using namespace std;

struct stu {
};
// sizeof stu

class sample {
};
// sizeof sample --> 1 - size:255

int  main()
{
	int *ptr = new int[1000];//sizeof(ptr) = 10*4 + 1
	*ptr = 100;
	cout << *ptr << endl;
	delete[] ptr;
}
