#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

int main() {
    std::string filename = "../data.csv";
    std::ifstream file(filename);

    if (!file.is_open()) {
        std::cerr << "Error: Could not open file " << filename << std::endl;
        return 1;
    }

    std::string line;
    // Skip the header
    std::getline(file, line);

    double total_sum = 0.0;
    int row_count = 0;

    // Read line by line
    while (std::getline(file, line)) {
        std::stringstream ss(line);
        std::string cell;
        
        // Parse ID
        std::getline(ss, cell, ',');
        // int id = std::stoi(cell); // Not used in calculation

        // Parse value1
        std::getline(ss, cell, ',');
        double v1 = std::stod(cell);

        // Parse value2
        std::getline(ss, cell, ',');
        double v2 = std::stod(cell);

        total_sum += (v1 * v2);
        row_count++;
    }

    std::cout << "C++ Data Processing Complete" << std::endl;
    std::cout << "Rows processed: " << row_count << std::endl;
    std::cout << "Total Sum of Products: " << total_sum << std::endl;

    return 0;
}
