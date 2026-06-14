consteval int computeMagicNumber() {
    int sum = 0;
    for(int i = 0; i < 100; i++) sum += i;
    return sum;
}

int main() {
    constexpr int magic = computeMagicNumber(); // Compiled as a raw integer literal!
}
