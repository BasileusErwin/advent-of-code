#include <fstream>
#include <ios>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

std::vector<std::vector<int>> read_file(const std::string &filename) {
  std::ifstream file(filename);
  if (!file) {
    std::cerr << "Error: failed to open file " << filename << std::endl;
    exit(1);
  }

  std::vector<std::vector<int>> reports;
  std::string line;
  while (std::getline(file, line)) {
    std::istringstream lineStream(line);
    std::vector<int> report;
    int level;
    while (lineStream >> level) {
      report.push_back(level);
    }
    reports.push_back(report);
  }
  return reports;
}

bool isSafe(const std::vector<int> &report) {
  bool increasing = true, decreasing = true;

  for (size_t i = 0; i < report.size() - 1; i++) {
    int diff = report[i + 1] - report[i];

    if (std::abs(diff) < 1 || std::abs(diff) > 3) {
      return false;
    }

    if (diff < 0) {
      increasing = false;
    } else if (diff > 0) {
      decreasing = false;
    }
  }

  return increasing || decreasing;
}

void part1(std::vector<std::vector<int>> reports) {
  int safeCount = 0;
  for (const auto &report : reports) {
    if (isSafe(report)) {
      safeCount++;
    }
  }

  std::cout << "Part1: " << safeCount << std::endl;
}

bool isSafeWithDampener(const std::vector<int> &report) {
  if (isSafe(report)) {
    return true;
  }

  for (size_t i = 0; i < report.size(); i++) {
    std::vector<int> modifiedReport = report;

    modifiedReport.erase(modifiedReport.begin() + i);

    if (isSafe(modifiedReport)) {
      return true;
    }
  }

  return false;
}

void part2(std::vector<std::vector<int>> reports) {
  int safeCount = 0;

  for (const auto &report : reports) {
    if (isSafeWithDampener(report)) {
      safeCount++;
    }
  }

  std::cout << "Part2: " << safeCount << std::endl;
}

int main() {
  std::vector<std::vector<int>> reports = read_file("input.txt");

  part1(reports);
  part2(reports);

  return 0;
}
