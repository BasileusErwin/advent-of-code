##[
  AoC: Day 4: Scratchcards
  URL: https://adventofcode.com/2023/day/4
]##
import std/strutils, std/re

const
  filename: string = "input.txt"

type Numbers = object
  winning: seq[int]
  numbers: seq[int]

proc getNumber(input: string): seq[Numbers] =
  let inputSplited: seq[string] = input.replace(re"Card \d+:", " ").split("\n")
  var numbers: seq[Numbers] = newSeq[Numbers]()

  for line in inputSplited:
    var lineSplited: seq[string] = line.strip().split(" ")

    var isWinningNumber: bool = true
    var winningNumber: seq[int] = newSeq[int]()
    var userNumbers: seq[int] = newSeq[int]()
    for i in lineSplited:
      if i == "":
        continue
      if i == "|":
        isWinningNumber = false
        continue

      if isWinningNumber:
        winningNumber.add(i.parseInt())
      else:
        userNumbers.add(i.parseInt())

    numbers.add(Numbers(winning: winningNumber, numbers: userNumbers))

  return numbers

proc part1(numbers: seq[Numbers]) =
  var totalPoints: int = 0
  for number in numbers:
    var points: int = 0
    for userNumber in number.numbers:
      if userNumber in number.winning:
        if points > 0:
          points = points * 2
        else:
          points = points + 1

    totalPoints = totalPoints + points

  echo("Part 1: ", totalPoints)

proc part2(numbers: seq[Numbers]) =
  echo("Part 2: ", 0)

proc main() =
  let file = open(filename)
  defer: file.close()

  let input: string = file.readAll()

  let numbers: seq[Numbers] = getNumber(input)

  part1(numbers)
  part2(numbers)

when isMainModule:
  main()
