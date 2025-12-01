import { lines } from "../util/lines";
import { runCli } from "../util/cli";
import { mapToInt } from "../util/int";
import { words } from "../util/words";
import { sum } from "../util/sum";
import { countGroups } from "../util/count";
import { test, expect } from "bun:test";


function turnDial(startPos: Number, instruction: string) {
  const [dir, ...digits] = instruction
  const rotation = (dir === 'R') ? Number(digits.join('')) : Number(digits.join(''))*-1
  let newPos = startPos+rotation

  let clicks = Math.abs(Math.floor(newPos/100))
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
    if (click) {
      clicks += 1
    }
  }
  return clicks
}


runCli({
    dir: import.meta.dir,
    part1,
    part2
});

// test("day01 part1 sample", () => {
//   expect(mapToInt(["1", "2", "3", "4 "])).toEqual([1, 2, 3, 4]);
// });

// test("day01 turnDial 11 R8", () => {
//   expect(turnDial(11, "R8")).toBe(19)
// });

// test("day01 turnDial 0 L1", () => {
//   expect(turnDial(0, "L1")).toBe(99)
// });

// test("day01 turnDial 99 R1", () => {
//   expect(turnDial(99, "R1")).toBe(0)
// });
