const dir = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
]

const xmas = 'XMAS'

export function part1(input: string): number {
  const grid = input.trim().split('\n').map((row) => row.split(''))
  let found = 0

  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] === xmas[0]) {
        let delta = checkXmas(grid, i, j)
        found = found + delta
      }
    }
  }

  return found
}

function checkXmas(grid: string[][], i: number, j: number): number {
  let found = 0

  for (const [dx, dy] of dir) {
    let x = i
    let y = j
    let k = 1
    while (k < xmas.length) {
      x += dx
      y += dy
      if (x < 0 || x >= grid.length || y < 0 || y >= grid[x].length || grid[x][y] !== xmas[k]) {
        break
      }
      k++
    }
    if (k === xmas.length) {
      found++
    }
  }

  return found
}

export function part2(input: string): number {

  return 2 * 4
}