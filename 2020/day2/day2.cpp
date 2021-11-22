#include <iostream>
#include <fstream>
#include <cstring>
#include <vector>

std::vector<std::string> split(std::string str, std::string delim) {
    std::vector<std::string> result;
    char* tokens = std::strtok(
            (char*) str.c_str(),
            (char*) delim.c_str());
    while (tokens != NULL) {
        result.push_back(std::string(tokens));
        tokens = std::strtok(NULL, (char*) delim.c_str());
    }
    return result;
}

int main() {
    std::ifstream infile("input.txt");
    std::string line;
    int result = 0;
    while (std::getline(infile, line)) {
        std::vector<std::string> split_line = split(line, " ");
        std::string range = split_line[0];
        std::vector<std::string> split_range = split(range, "-");
        int start = std::stoi(split_range[0]);
        int end = std::stoi(split_range[1]);
        char val = split_line[1][0];
        std::string password = split_line[2];

        if (
            ((start - 1) >= 0) && 
            ((end - 1) >= 0) && 
            ((password[start - 1] == val && password[end - 1] != val) || 
            (password[start - 1] != val && password[end - 1] == val))
        ) {
            ++result;
        }
    }

    std::cout << result << std::endl;
}