#include <algorithm>
#include <fstream>
#include <ios>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

std::string read_file(std::string filename) {
  std::ifstream file(filename);

  if (!file) {
    std::cerr << "Error: failed to open file " << filename << std::endl;
    exit(1);
  }

  std::stringstream buffer;
  buffer << file.rdbuf();
  return buffer.str();
}

void part1(std::vector<int> leftColumns, std::vector<int> rightColumns) {
  int total = 0;
  for (size_t i = 0; i < leftColumns.size(); i++) {
    total += std::abs(leftColumns[i] - rightColumns[i]);
  }

  std::cout << "Part 1: " << total << std::endl;
}

void part2(std::vector<int> leftColumns, std::vector<int> rightColumns) {
  std::unordered_map<int, int> map;

  for (size_t i = 0; i < leftColumns.size(); i++) {
    map[leftColumns[i]] = 0;

    for (size_t j = 0; j < rightColumns.size(); j++) {
      if (leftColumns[i] == rightColumns[j]) {
        map[leftColumns[i]]++;
      }
    }
  }

  int total = 0;

  for (auto it = map.begin(); it != map.end(); it++) {
    total += it->first * it->second;
  }

  std::cout << "Part 2: " << total << std::endl;
}

int main() {
  std::string input = read_file("input.txt");

  std::vector<int> leftColumns;
  std::vector<int> rightColumns;

  std::istringstream stream(input);
  std::string line;
  while (std::getline(stream, line)) {
    std::istringstream lineStream(line);
    int left, right;

    if (lineStream >> left >> right) {
      leftColumns.push_back(left);
      rightColumns.push_back(right);
    }
  }

  std::sort(leftColumns.begin(), leftColumns.end());
  std::sort(rightColumns.begin(), rightColumns.end());

  part1(leftColumns, rightColumns);
  part2(leftColumns, rightColumns);
}
