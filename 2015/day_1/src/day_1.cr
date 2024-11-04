# AOC 2015 Day 1
# https://adventofcode.com/2015/day/1

input : String = File.read("./input.txt")

input_splited : Array(String) = input.split("")

current_floor : Int32 = 0
current_instruction : Int32 = 0
index : Int32 = 0
while index < input_splited.size
  if input_splited[index] == "("
    current_floor += 1
  elsif input_splited[index] == ")"
    current_floor -= 1
  end

  index += 1

  if current_floor == -1 && (current_instruction == 0)
    current_instruction = index
  end
end

puts "Part 1: #{current_floor}"
puts "Part 2: #{current_instruction}"
