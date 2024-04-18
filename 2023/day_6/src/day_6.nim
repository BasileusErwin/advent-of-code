#[

]#

import strutils
import sequtils

const fileName: string = "input.txt"

type InputType = 
  tuple[times: seq[int], distances: seq[int]]

proc processInput(input: string): InputType =
  var
    lines: seq[string] = input.strip().split("\n").map(proc (x: string): string = x.split(":")[1].strip())
    time: seq[int] = lines[0].split(" ").filter(proc (x: string): bool = x != "").map(proc (x: string): int = x.strip().parseInt())
    distance: seq[int] = lines[1].split(" ").filter(proc (x: string): bool = x != "").map(proc (x: string): int = x.strip().parseInt())

  return (time, distance)

proc part1(data: InputType) = 
  var 
    times: seq[int] = data.times
    distances: seq[int] = data.distances
    total: int = 1

  for i in 0 ..< times.len:
    var 
      time: int = times[i]
      distance: int = distances[i]
      winningWays: int = 0

    for j in 0 ..< time:
      var
        speed: int = j
        travelTime: int = time - j
        distanceTravelled: int = speed * travelTime
      if distanceTravelled > distance:
        winningWays += 1

    total *= winningWays

  echo total

proc main() =
  var input: string = ""
  try:
    let file = open(fileName)
    defer: file.close()

    input = file.readAll()
  except: 
    echo "No file found"

  var data: InputType = processInput(input)
  part1(data)


when isMainModule:
  main()
