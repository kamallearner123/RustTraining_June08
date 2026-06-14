#include <iostream>
#include <memory>
using namespace std;

class stud {
public:
int data;
stud()
{
	cout << "Constructor Used" << endl;
}
~stud()
{
cout << "Destructor Used" << endl;
}
};

std::unique_ptr <stud > fun()
{
	std::unique_ptr <stud > ptr(new stud);
	cout << "End of the function" << endl;
	return ptr;
}
int main()
{
	std::unique_ptr <stud > ptr = fun();
	std::unique_ptr <stud > ptr1 = move(ptr);
	cout << "End of the program" << endl;

	stud *raw_ptr = ptr1.release();

}
