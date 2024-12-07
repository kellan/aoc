
export function part1(input: string): number {

  const rows = input.trim().split("\n").map((row) => {
    return row.trim().split(/\s+/).map(Number)
  })

  const [left, right] = [
    rows.map((row) => row[0]).sort(),
    rows.map((row) => row[1]).sort()
  ]

  return left.reduce(
    (acc, curr, i) => acc + Math.abs(curr - right[i]),
    0
  )
}

export function part2(input: string): number {
  const lines = input.split("\n")
  return lines.length * 2;
}