import { lines } from "../util/lines";
import { runCli, sampleText } from "../util/cli";
import { sum } from "../util/sum";

export function part1(input: string) {
    let largest:number[] = []
    for (const l of lines(input)) {
        const digits = l.split("").map(Number)
        const maxDigit = Math.max(...digits.slice(0,-1))
        const maxIndex = digits.indexOf(maxDigit)
        const tail = digits.slice(maxIndex+1)
        const secondMax = Math.max(...tail)
        largest.push(maxDigit*10+secondMax)
    }
    return sum(largest)
}

export function part2(input: string) {
}

if (import.meta.main && process.env.NODE_ENV !== "test") {
  
  runCli({
      dir: import.meta.dir,
      part1,
      part2
  });
}


if (import.meta.main && process.env.NODE_ENV == "test") {
  const input = sampleText(import.meta.dir)
  let largest:number[] = []
  for (const l of lines(input)) {
    const digits = l.split("").map(Number)
    const maxDigit = Math.max(...digits.slice(0,-1))
    const maxIndex = digits.indexOf(maxDigit)
    const tail = digits.slice(maxIndex+1)
    const secondMax = Math.max(...tail)
    largest.push(maxDigit*10+secondMax)
  }
  
  test("day03 sample part1", () => {
    expect(largest).toEqual([98,89,78,92])
  })
}
