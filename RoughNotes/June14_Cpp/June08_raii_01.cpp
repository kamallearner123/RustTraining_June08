#include <iostream>
#include <string>

void riskyFunction() {
    FILE* file = fopen("data.txt", "r");
    // ... some code that might throw
    if (1) {
        //throw std::runtime_error("Error occurred .... ");
    }
    //fclose(file);  // Might never be reached!
}


int main() {
	riskyFunction();
}
