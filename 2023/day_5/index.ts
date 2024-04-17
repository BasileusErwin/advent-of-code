/*
 * AoC 2023: Day 5
 * */
import * as fs from "fs";
import * as path from "path";

function loadFile(): string {
  const filePath = path.join(__dirname, "input.txt");

  return fs.readFileSync(filePath, "utf8");
}

function process(maps: number[][], index: number, value: number): number {
  if (index == maps.length) {
    return value;
  }

  const [destination, source, range] = maps[index];
  if (source <= value && value < source + range) {
    return process(
      maps,
      index + maps.slice(index + 1).findIndex((x) => x.length < 3),
      destination + value - source,
    );
  }

  return process(maps, index + 1, value);
}

(() => {
  const input: string[] = loadFile().split("\n");

  const seeds: number[] = input[0].split(": ")[1].split(" ").map(Number);
  const maps: number[][] = input.slice(2).map((x) => x.split(" ").map(Number));

  const result: number[] = seeds.map((x) => process(maps, 0, x));

  console.log(Math.min(...result));
})();
