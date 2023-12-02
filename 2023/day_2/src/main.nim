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

# calculatePowerOfSet: Calculates the "power" of an individual set.
# The power is defined as the product of the maximum number of red, green and blue cubes used in any set in the set.
# used in any set in the game. This function determines the maximum number of cubes
# of each color required in the sets of the game and returns its product.
#
# Parameters:
#
# game: a Game object containing the sets of cubes to evaluate.
#
# Returns: The power of the game as an integer.
proc calculatePowerOfSet(game: Game): int =
  var
    red: int = 0
    green: int = 0
    blue: int = 0

  for set in game.sets:
    if set.red > red:
      red = set.red
    if set.green > green:
      green = set.green
    if set.blue > blue:
      blue = set.blue

  return red * green * blue

# calculatePowerOfGames: Calculates the total combined power of a sequence of games.
# Iterates over each game, calculating its individual power with the function calculatePowerOfSet
# and adding these powers to get the total.
#
# Parameters:
# games: a sequence of Game objects to evaluate.
#
# Returns: The combined total power of all games as an integer.
proc calculatePowerOfGames(games: seq[Game]): int =
  var power: int = 0

  for game in games:
    power += calculatePowerOfSet(game)

  return power

when isMainModule:
  let games = loadFileAndPrecess()
  let totalGames: int = getTotalGamesIsPossible(games)
  let powerOfGame: int = calculatePowerOfGames(games)

  echo(fmt("Part 1: Total games {totalGames}"))
  echo(fmt("Part 2: Power of game {powerOfGame}"))
