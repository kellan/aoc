import { lines } from "../util/lines";
import { runCli } from "../util/cli";
import { range } from "../util/range";
import { sum } from "../util/sum";
import { str_chunk } from "../util/chunk";
import { join } from "node:path";
import { readFileSync } from "node:fs";


export function part1(input: string) {
  let ids:number[] = []

  lines(input)[0]?.split(',').map(seq => {
    
    let [s,e] = seq.split('-').map(n => Number(n))
    for (const n of range(s,e)) {
      if (isInvalid1(n)) {
        ids.push(n)
      }
    }
  })
  return sum(ids)
}

function isInvalid1(id: number) {
  const s = String(id)
  if (s.slice(0, s.length/2) == s.slice(s.length/2)) {
    return true
  } else {
    return false
  }
}

export function part2(input: string) {
  let ids:number[] = []
  lines(input)[0]?.split(',').map(seq => {
    let [s,e] = seq.split('-').map(n => Number(n))
    for (const n of range(s,e)) {
      if (isInvalid2(n)) {
        ids.push(n)
      }
    }
  })
  //console.log(ids)
  return sum(ids)
}

function isInvalid2(id: number) {
  const s = String(id)
  const re = /^(\d+)\1+$/;
  return re.test(s)


}

if (import.meta.main && process.env.NODE_ENV !== "test") {
  
  runCli({
      dir: import.meta.dir,
      part1,
      part2
  });
}

if (import.meta.main && process.env.NODE_ENV == "test") {

  const sampleTxt = join(import.meta.dir, 'sample.txt');
  const input = readFileSync(sampleTxt, "utf8");
  let ids:number[] = []

  lines(input)[0]?.split(',').map(seq => {
  let [s,e] = seq.split('-').map(n => Number(n))
  for (const n of range(s,e)) {
    if (isInvalid2(n)) {
      ids.push(n)
    }
  }})  
  
  test("day02 sample part2", () => {
    expect(ids.sort((a, b) => a - b))
      .toEqual([11,22,99,111,999,1010,1188511885,222222,446446,38593859,565656,824824824,2121212121]
      .sort((a, b) => a - b))
      })
}