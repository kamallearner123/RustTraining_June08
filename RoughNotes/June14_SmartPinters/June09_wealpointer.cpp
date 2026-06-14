#include <iostream>
#include <memory>

class B;

class A {
public:
    std::weak_ptr<B> b;
    ~A() { std::cout << "A destroyed\n"; }
};

class B {
public:
    std::weak_ptr<A> a;
    ~B() { std::cout << "B destroyed\n"; }
};

int main()
{
    auto objA = std::make_shared<A>();
    auto objB = std::make_shared<B>();

    //std::weak_ptr<A> = objA; ////// save the pointer temp

    objA->b = objB;
    objB->a = objA;


}
