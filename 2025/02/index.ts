import { lines } from "../util/lines";
import { runCli } from "../util/cli";
import { range } from "../util/range";
import { sum } from "../util/sum";


export function part1(input: string) {
  const [line] = lines(input)
  //console.log(line)
  let ids = []
  line.split(',').map(seq => {
    let [s,e] = seq.split('-').map(n => Number(n))
    for (const n of range(s,e)) {
      if (isInvalid(n)) {
        ids.push(n)
      }
    }
  })
  return sum(ids)
}

function isInvalid(id: number) {
  const s = String(id)
  if (s.slice(0, s.length/2) == s.slice(s.length/2)) {
    return true
  } else {
    return false
  }
}

export function part2(input: string) {
  return -1
}


if (import.meta.main && process.env.NODE_ENV !== "test") {
  
  runCli({
      dir: import.meta.dir,
      part1,
      part2
  });
}