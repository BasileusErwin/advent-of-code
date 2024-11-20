#include <fstream>
#include <iostream>
#include <set>
#include <string>

using namespace std;

std::string read_file(std::string filename) {
  std::ifstream file(filename);
  std::string content;
  std::string line;

  while (std::getline(file, line)) {
    content += line;
    content += "\n";
  }

  return content;
}

/**
 * ^ - north
 * v - south
 * > - east
 * < - west
 */
void move(char movement, int *x, int *y) {
  switch (movement) {
  case '^':
    (*y)++;
    break;
  case 'v':
    (*y)--;
    break;
  case '>':
    (*x)++;
    break;
  case '<':
    (*x)--;
    break;
  default:
    break;
  }
}

void part1(std::string input) {
  std::set<std::pair<int, int>> visited;
  int x = 0;
  int y = 0;

  visited.insert({x, y});

  for (char movement : input) {
    move(movement, &x, &y);
    visited.insert({x, y});
  }

  std::cout << "Part 1: " << visited.size() << std::endl;
}

void part2(std::string input) {
  std::set<std::pair<int, int>> visited;

  int santaX = 0, santaY = 0;
  int roboX = 0, roboY = 0;

  visited.insert({0, 0});

  for (size_t i = 0; i < input.size(); i++) {
    if (i % 2 == 0) {
      move(input[i], &santaX, &santaY);
      visited.insert({santaX, santaY});
    } else {
      move(input[i], &roboX, &roboY);
      visited.insert({roboX, roboY});
    }
  }

  std::cout << "Part 2: " << visited.size() << std::endl;
}

int main() {
  std::string input = read_file("input.txt");

  part1(input);
  part2(input);

  return 0;
}
