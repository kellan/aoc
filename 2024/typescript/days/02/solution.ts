
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
  //console.log("Level", level, "is safe")
  return 1
}

export function part1(input: string): number {

  const rows = input.trim().split("\n").map((row) => {
    return row.trim().split(/\s+/).map(Number)
  })

  //console.log(`Rows length ${rows.length}`)
  return rows.reduce((acc, curr) => acc + isReportSafe(curr), 0)

}

export function part2(input: string): number {
  const rows = input.trim().split("\n").map((row) => {
    return row.trim().split(/\s+/).map(Number)
  })
  let safe = 0
  for (const row of rows) {
    if (isReportSafe(row)) {
      safe++
    } else {
      for (let i = 0; i < row.length; i++) {
        const shortRow = row.slice(0, i).concat(row.slice(i + 1))
        if (isReportSafe(shortRow)) {
          //console.log(`Report ${row} is not safe, but ${shortRow} is`)
          safe++
          break
        }
      }
    }
  }

  return safe
}