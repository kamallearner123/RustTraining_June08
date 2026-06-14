#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>

int main(int argc, char* argv[]) {
    std::cout << "=== WASI Lab Demo ===" << std::endl;

    // 1. Reading Environment Variables
    // The host must explicitly export these; WASI can't read arbitrary env vars.
    const char* user_env = std::getenv("LAB_USER");
    std::string user = user_env ? user_env : "Guest";
    const char* mode_env = std::getenv("LAB_MODE");
    std::string mode = mode_env ? mode_env : "default";
    std::cout << "[ENV] User: " << user << ", Mode: " << mode << std::endl;

    // 2. Reading a File (host must grant directory access via --dir flag)
    std::string input_path = "data/input.txt";
    std::ifstream infile(input_path);
    if (infile.is_open()) {
        std::string contents((std::istreambuf_iterator<char>(infile)), std::istreambuf_iterator<char>());
        std::cout << "[FILE READ] Content:\n" << contents;
        infile.close();
    } else {
        std::cout << "[FILE READ] Error (is --dir granted?)" << std::endl;
    }

    // 3. Writing a File
    std::string output_path = "data/output.txt";
    std::ofstream outfile(output_path);
    if (outfile.is_open()) {
        outfile << "Processed by WASI module for user: " << user << "\n";
        std::cout << "[FILE WRITE] Written to " << output_path << std::endl;
        outfile.close();
    } else {
        std::cout << "[FILE WRITE] Error writing to " << output_path << std::endl;
    }

    // 4. Reading command-line arguments
    std::cout << "[ARGS] Received " << (argc - 1) << " argument(s): [";
    for (int i = 1; i < argc; ++i) {
        std::cout << "\"" << argv[i] << "\"";
        if (i < argc - 1) std::cout << ", ";
    }
    std::cout << "]" << std::endl;

    return 0;
}
