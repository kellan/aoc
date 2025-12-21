import { lines } from "../util/lines";
import { runCli, sampleText } from "../util/cli";


export function part1(input: string) {
    const inputs = input.split("\n\n")
    const ranges = lines(inputs[0]).map(s => s.split("-").map(Number))
    const ingredients = lines(inputs[1]).map(Number)
    const fresh = []

    for (const i of ingredients) {
        for (const r of ranges) {
            if (i >= r[0] && i <= r[1]) {
                fresh.push(i)
                break
            }
        }
    }

    return fresh.length
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
