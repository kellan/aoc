import { lines } from "../util/lines";
import { runCli, sampleText } from "../util/cli";
import { sum } from "../util/sum";

export function part1(input: string) {
    let largest:number[] = []
    for (const l of lines(input)) {
        largest.push(largestLeftInt(l, 2))
    }
    return sum(largest)
}

export function part2(input: string) {
  let largest:number[] = []
  for (const l of lines(input)) {
      largest.push(largestLeftInt(l, 12))
  }
  return sum(largest)
}

function largestLeftInt(input: string, size: number = 2) {
  let answer = []
  let start = 0
  for(let i=1; i<=size; i++) {
    let [maxInt, maxIndex] = largestLeftDigit(input, start, size-i)
    start = maxIndex+1
    answer.push(maxInt)
  }
  
  return Number(answer.join(""))
}

function largestLeftDigit(input: string, start: number = 0, remaining: number = 1) {
  let digits = input.split("").map(Number)
  let search = remaining > 0 ? digits.slice(start, -remaining) : digits.slice(start)
  const maxDigit = Math.max(...search)
  const maxIndex = search.indexOf(maxDigit)+start
  //console.log(start, remaining, search, maxDigit, maxIndex)
  return [maxDigit, maxIndex]
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

  let answers2 = []

  for (const l of lines(input)) {
    answers2.push(largestLeftInt(l, 2))
  }

  test("day 03 sample largestLeft 2 batteries", () => {
    expect(answers2[0]).toEqual(98)
    expect(answers2[1]).toEqual(89)
    expect(answers2[2]).toEqual(78)
    expect(answers2[3]).toEqual(92)
  })


  let answers12 = []

  for (const l of lines(input)) {
    answers12.push(largestLeftInt(l, 12))
  }
  
  test("day 03 sample largestLeft 12 batteries", () => {
    expect(answers12[0]).toEqual(987654321111)
    expect(answers12[1]).toEqual(811111111119)
    expect(answers12[2]).toEqual(434234234278)
    expect(answers12[3]).toEqual(888911112111)
  })

  let part1sample = part1(input)
  test("day 03 sample part1", () => {
    expect(part1sample).toEqual(357)
  })

  let part2sample = part2(input)
  test("day 03 sample part2", () => {
    expect(part2sample).toEqual(3121910778619)
  })
}
