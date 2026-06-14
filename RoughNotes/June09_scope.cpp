#include <iostream>
#include <stdio.h>
using namespace std;

class stud_uniq {
	FILE *fd;
	int *ptr;
	public:
		stud_uniq(int *num) {
			//ptr = new int[num];
			ptr = num;
			cout << "Constructor " << endl;
			fd = fopen("file.txt", "r");	
		}
		~stud_uniq() {
			delete ptr;
			cout << "Destructor " << endl;
			fclose(fd);
		}

};

void fun() {
	stud_uniq s1{new int};
}

int main() {
	fun();
	cout << "End of the program!!!" << endl;
}


