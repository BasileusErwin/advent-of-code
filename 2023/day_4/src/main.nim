##[
  AoC: Day 4: Scratchcards
  URL: https://adventofcode.com/2023/day/4
]##
import std/strutils, std/re

const
  filename: string = "input.txt"

type Numbers = object
  id: int
  winning: seq[int]
  numbers: seq[int]

proc getNumber(input: string): seq[Numbers] =
  let inputSplited: seq[string] = input.replace(re"Card \d+:", " ").split("\n")
  var numbers: seq[Numbers] = newSeq[Numbers]()

  var id: int = -1
  for line in inputSplited:
    var lineSplited: seq[string] = line.strip().split(" ")

    id = id + 1
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

    numbers.add(Numbers(id: id, winning: winningNumber, numbers: userNumbers))

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

proc countingCoincidences(card: Numbers): int =
  var coincidences: int = 0

  for userNumber in card.numbers:
    if userNumber in card.winning:
      coincidences = coincidences + 1

  return coincidences

proc calculateTotalCards(cards: seq[Numbers]): int =
  var
    totalCards = -1
    queueCard = cards

  while queueCard.len != 0:
    let currentCard: Numbers  = queueCard.pop()
    let coincidences: int = countingCoincidences(currentCard)
    totalCards += 1

    for i in 1 .. coincidences:
      if currentCard.id + i < cards.len:
        queueCard.add(cards[currentCard.id + i])

  return totalCards

proc part2(numbers: seq[Numbers]) =
  echo(calculateTotalCards(numbers))

proc main() =
  let file = open(filename)
  defer: file.close()

  let input: string = file.readAll()

  let numbers: seq[Numbers] = getNumber(input)

  part1(numbers)
  part2(numbers)

when isMainModule:
  main()
