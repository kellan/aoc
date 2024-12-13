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
        let delta = checkXmas(xmas, grid, i, j)
        found = found + delta
      }
    }
  }

  return found
}

function checkXmas(word: string, grid: string[][], i: number, j: number): number {
  let found = 0

  for (const [dx, dy] of dir) {
    let x = i
    let y = j
    let k = 0
    while (k < xmas.length) {

      if (x < 0 || x >= grid.length || y < 0 || y >= grid[x].length || grid[x][y] !== xmas[k]) {
        break
      }
      x += dx
      y += dy
      k++
    }
    if (k === xmas.length) {
      found++
    }
  }

  return found
}

const dir2 = [
  [-1, -1],
  [-1, 1],
  [1, -1],
  [1, 1],
]

const word = 'MAS'

export function part2(input: string): number {
  const grid = input.trim().split('\n').map((row) => row.split(''))

  let found = 0

  for (let i = 1; i < grid.length - 1; i++) {
    for (let j = 1; j < grid[i].length - 1; j++) {

      if (grid[i][j] === 'A') {

        const diagonals = [
          grid[i - 1][j - 1],
          grid[i - 1][j + 1],
          grid[i + 1][j - 1],
          grid[i + 1][j + 1],
        ].join('')

        if (['MSMS', 'MMSS', 'SSMM', 'SMSM'].includes(diagonals)) {
          found++
        }
      }
    }
  }

  return found
}