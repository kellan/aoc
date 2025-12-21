import { lines } from "../util/lines";
import { runCli, sampleText } from "../util/cli";
import { sum } from "../util/sum";


export function part1(input: string) {
    const inputs = input.split("\n\n")
    const ranges = lines(inputs[0]).map(s => s.split("-").map(Number))
    const ingredients = lines(inputs[1]).map(Number)
    const fresh = []

    for (const i of ingredients) {
        for (const r of ranges) {
            if (contains(i, r)) {
                fresh.push(i)
                break
            }
        }
    }

    return fresh.length
}

function contains(i: number, r: number[]): boolean {
    return i >= r[0] && i <= r[1]
}

export function part2(input: string) {
    const inputs = input.split("\n\n")
    const ranges = lines(inputs[0]).map(s => s.split("-").map(Number))

    ranges.sort((a, b) => a[0] - b[0]);

    const S = 0
    const E = 1

    let consolidated = []

    for (let i=0;i<ranges.length;i++) {
        if (i == ranges.length-1) {
            consolidated.push(ranges[i])
        } else if (ranges[i][E] < ranges[i+1][S]) {
            consolidated.push(ranges[i])
        } else {
            ranges[i+1][S] = ranges[i][S]
            ranges[i+1][E] = Math.max(ranges[i][E], ranges[i+1][E])
        }
    }

    return sum(consolidated.map(r => r[1] - r[0] + 1));

}

if (import.meta.main && process.env.NODE_ENV !== "test") {
  
  runCli({
      dir: import.meta.dir,
      part1,
      part2
  });
}
