
function isReportSafe(level: number[]): number {
  let lastDiff = 0
  for (let i = 1; i < level.length; i++) {
    const diff = level[i] - level[i - 1]

    if (Math.abs(diff) < 1 || Math.abs(diff) > 3) {
      return 0
    }

    if (i > 1 && (lastDiff * diff < 0)) {
      return 0
    }

    lastDiff = diff
  }
  return 1
}

export function part1(input: string): number {

  const rows = input.trim().split("\n").map((row) => {
    return row.trim().split(/\s+/).map(Number)
  })

  return rows.reduce((acc, curr) => acc + isReportSafe(curr), 0)

}

export function part2(input: string): number {
  return 2

}