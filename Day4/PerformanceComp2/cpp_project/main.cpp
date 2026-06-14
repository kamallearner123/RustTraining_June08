#include <iostream>
#include <fstream>
#include <string>
#include <unordered_map>
#include <vector>
#include <algorithm>

int main() {
    std::string filename = "words.txt";
    std::ifstream file(filename);

    if (!file.is_open()) {
        std::cerr << "Error: Could not open file " << filename << std::endl;
        return 1;
    }

    std::unordered_map<std::string, int> word_counts;
    std::string word;

    int total_words = 0;

    // Read word by word
    while (file >> word) {
        word_counts[word]++;
        total_words++;
    }

    // Convert to vector for sorting
    std::vector<std::pair<std::string, int>> sorted_counts(word_counts.begin(), word_counts.end());

    // Sort descending by frequency
    std::sort(sorted_counts.begin(), sorted_counts.end(), [](const auto& a, const auto& b) {
        return a.second > b.second;
    });

    std::cout << "C++ Hash Map Processing Complete" << std::endl;
    std::cout << "Total Words Processed: " << total_words << std::endl;
    std::cout << "Unique Words Found: " << word_counts.size() << std::endl;
    
    std::cout << "Top 5 Words:" << std::endl;
    for (int i = 0; i < 5 && i < sorted_counts.size(); ++i) {
        std::cout << "  " << sorted_counts[i].first << ": " << sorted_counts[i].second << std::endl;
    }

    return 0;
}
