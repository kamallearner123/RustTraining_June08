#include <iostream>
using namespace std;
class stud {
public:
stud()
{
cout << "Constructor Used" <<
endl;
}
~stud()
{
cout << "Destructor Used" <<
endl;
}
};
int main()
{
stud* S = new stud[6];
delete[] S;
}
