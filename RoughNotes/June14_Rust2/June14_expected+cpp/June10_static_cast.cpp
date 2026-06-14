#include <iostream>

class Base {};
class Derived : public Base {};

int main() {
    // 1. Primitive conversion
    int i = 100;
    double d = static_cast<double>(i);        // OK

    // 2. Upcast (always safe)
    Derived der;
    Base* b = static_cast<Base*>(&der);       // OK

    // 3. Downcast (unsafe if wrong type!)
    Base* b2 = new Derived();
    Derived* d2 = (Derived*)b2;  // OK - you promise it's correct

    // 4. void* conversion
    void* v = &i;
    int* p = static_cast<int*>(v);            // OK

    return 0;
}
