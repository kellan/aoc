import { lines } from "../util/lines";
import { runCli } from "../util/cli";
import { mapToInt } from "../util/int";
import { words } from "../util/words";
import { sum } from "../util/sum";
import { countGroups } from "../util/count";
import { test, expect } from "bun:test";


function turnDial(startPos: Number, instruction: string) {

  const [dir, digits] = [instruction[0], Number(instruction.slice(1))]
  const rotation = (dir === 'R') ? digits : digits*-1
  const newPos = (startPos + rotation + 100) % 100;
  let clicks = 0
  if (rotation > 0) {
    clicks = Math.floor((startPos + rotation) / 100) - Math.floor(startPos / 100) 
  } else {
    clicks = Math.floor((startPos - 1) / 100) - Math.floor((startPos - 1 + rotation) / 100)
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
    //console.log(l, pos, click, clicks)
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

if (import.meta.main && process.env.NODE_ENV == "test") {
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
  
  const sample = `
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
`.trim();

test("day01 sample part1", () => {
  expect(part1(sample)).toEqual(3)
})

test("d1 R48", () => {
  expect(turnDial(52, "R48")).toEqual([0, 1])
})

test("d1 R60", () => {
  expect(turnDial(95, "R60")).toEqual([55, 1])
})

test("d1 L55", () => {
  expect(turnDial(55, "L55")).toEqual([0, 1])
})

test("d1 L99", () => {
  expect(turnDial(99, "L99")).toEqual([0, 1])
})

test("d1 L82", () => {
  expect(turnDial(14, "L82")).toEqual([32, 1])
})

test("day01 sample part2", () => {
  expect(part2(sample)).toEqual(6)
})

}


