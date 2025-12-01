import { lines } from "../util/lines";
import { runCli } from "../util/cli";
import { mapToInt } from "../util/int";
import { words } from "../util/words";
import { sum } from "../util/sum";
import { countGroups } from "../util/count";

export function part1(input: string) {
  //console.log("TODO 1");
  const data = lines(input, (l) => mapToInt(words(l)));
  const [left, right] = [
    data.map((row) => row[0]).sort(),
    data.map((row) => row[1]).sort()
  ]
  const diffSum = sum(left.map((v, i) => Math.abs(right[i] - v)))
  return diffSum;
}

export function part2(input: string) {
  const data = lines(input, (l) => mapToInt(words(l)));
  const [left, right] = [
    data.map((row) => row[0]).sort(),
    data.map((row) => row[1]).sort()
  ]
  const counts = countGroups(right);
  const similarity = left.map((v, i) => v * (counts[v] ?? 0));
  return sum(similarity)
}


runCli({
    dir: import.meta.dir,
    part1,
    part2
});
