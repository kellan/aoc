import { lines } from "../util/lines";
import { runCli } from "../util/cli";
import { mapToInt } from "../util/int";
import { words } from "../util/words";
import { sum } from "../util/sum";
import { countGroups } from "../util/count";
import { test, expect } from "bun:test";


export function turnDial(startPos: Number, instruction: string) {
  const [dir, digits] = [instruction[0], Number(instruction.slice(1))]
  const rotation = (dir === 'R') ? digits : digits*-1

  let newPos = startPos+rotation
  let clicks = Math.floor(Math.abs(newPos)/100)
  newPos = newPos % 100
  if (newPos < 0) {
    newPos = 100 + newPos
  }
  
  return [newPos, clicks]
}

export function part1(input: string) {
  let pos = 50
  let zeroes = 0
  for (const l of lines(input)) {
    [pos] = turnDial(pos, l)
    if (pos === 0) {
      zeroes += 1
    }
  }
  return zeroes

}

export function part2(input: string) {
  let pos = 50
  let click = false
  let clicks = 0
  for (const l of lines(input)) {
    [pos, click] = turnDial(pos, l)
    clicks += click
    console.log(l, pos, click, clicks)
  }
  return clicks
}


if (import.meta.main && process.env.NODE_ENV !== "test") {
  runCli({
      dir: import.meta.dir,
      part1,
      part2
  });
}

test("day01 part1 sample", () => {
  expect(mapToInt(["1", "2", "3", "4 "])).toEqual([1, 2, 3, 4]);
});

test("day01 turnDial 11 R8", () => {
  expect(turnDial(11, "R8")).toEqual([19,0])
});

test("day01 turnDial 0 L1", () => {
  expect(turnDial(0, "L1")).toEqual([99,0])
});

test("day01 turnDial 99 R1", () => {
  expect(turnDial(99, "R1")).toEqual([0,1])
});

test("day01 turnDial 50 R1000", () => {
  expect(turnDial(50, "R1000")).toEqual([50,10])
});

// 1 1 2 2 3 4 4 5 6