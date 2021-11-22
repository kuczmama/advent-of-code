#include <iostream>
#include <fstream>
#include <set>

int main() {
    std::ifstream infile("input.txt");
    std::string line;
    std::set<int> set;
    int goal = 2020;
    while (std::getline(infile, line)) {
        int num = stoi(line);
        int needed = goal - num;
        // goal = num1 + num2 + num3
        set.insert(num);
    }

    for (std::set<int>::iterator it = set.begin(); it != set.end(); it++) {
        int num1 = *it;
        for (std::set<int>::iterator it2 = set.begin(); it2 != set.end(); it2++) {
            int num2 = *it2;
            int needed = goal - (num1 + num2);
            if (num1 != num2 && set.count(needed)) {
                // std::cout << "num1: " << num1 << " num2: " << num2 << " num3: " << needed << std::endl;
                std::cout << needed * num2 * num1 << std::endl;
            }
        }
    }
}