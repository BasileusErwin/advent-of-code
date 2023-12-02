##[
  AoC: Day 2: Cube Conundrum
  URL: https://adventofcode.com/2023/day/2
]##
import strutils
import strformat

# Constants to define the total number of cubes of each color.
const
  totalCubeRed = 12
  totalCubeGreen = 13
  totalCubeBlue = 14
  filename = "input.txt"

# Definition of types to represent cubes and sets.
type
  Cube = object
    red, green, blue: int

  Game = object
    id: int
    sets: seq[Cube]

# Function to process an input line and convert it into a Game object
# Extracts the Game ID and details of each set of cubes
proc processStringInput(input: string): Game =
  var inputSplited = input.split(":")
  var sets = newSeq[Cube]()

  for set in inputSplited[1].split(";"):
    var cube = Cube(red: 0, green: 0, blue: 0)

    for color in set.split(","):
      var value = color.split(" ")[1].parseInt()

      case color.split(" ")[2]
        of "red":
          cube.red = value
        of "green":
          cube.green = value
        of "blue":
          cube.blue = value

    sets.add(cube)

  return Game(id: inputSplited[0].split(" ")[1].parseInt(), sets: sets)

proc loadFileAndPrecess(): seq[Game] =
  try:
    let file = open(filename)
    defer: file.close()

    var games = newSeq[Game]()
    for line in file.lines:
      games.add(processStringInput(line))

    return games
  except:
    echo("Error reading file")

# Part 1

# Function to determine the total possible sets given the cube constraints
proc getTotalGamesIsPossible(games: seq[Game]): int =
  var gameIds: seq[int] = newSeq[int]()

  for game in games:
    var isCrrectGame: bool = true
    for set in game.sets:
      if set.red > totalCubeRed or set.green > totalCubeGreen or set.blue > totalCubeBlue:
        isCrrectGame = false
        break

    if isCrrectGame:
      gameIds.add(game.id)

  var totalGames: int = 0
  for id in gameIds:
    totalGames += id

  return totalGames

# Part 2

proc calculatePowerOfCube(cube: Cube): int =
  return 0

proc calculatePowerOfGames(games: seq[Game]): int =
  return 0

when isMainModule:
  let games = loadFileAndPrecess()
  let totalGames: int = getTotalGamesIsPossible(games)
  let powerOfGame: int = calculatePowerOfGames(games)

  echo(fmt("Part 1: Total games {totalGames}"))
  echo(fmt("Part 2: Power of game {powerOfGame}"))
