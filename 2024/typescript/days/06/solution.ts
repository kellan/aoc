const dir = [
    [-1, 0],
    [0, 1],
    [1, 0],
    [0, -1],
]

type Position = {
    x: number,
    y: number,
    dir: number
}

export function part1(input: string): number {
    const grid = input.trim().split('\n').map((row) => row.split(''))
    let guardRow = grid.findIndex((row) => row.includes('^'));
    let guardCol = grid[guardRow].indexOf('^');

    let visited: Map<String, Position> = new Map()

    let currentDir = 0

    run(grid, guardRow, guardCol, visited, currentDir)
    console.log(visited)
    return visited.size
}

function run(grid: string[][],
    guardRow: number, guardCol: number,
    visited: Map<string, Position>,
    currentDir: number): boolean {

    let isLoop = false
    visited.set(`${guardRow},${guardCol}`, { x: guardRow, y: guardCol, dir: currentDir })

    while (true) {
        let dx = dir[currentDir][0]
        let dy = dir[currentDir][1]
        let nextRow = guardRow + dx
        let nextCol = guardCol + dy

        if (grid[nextRow] == undefined || grid[nextRow][nextCol] == undefined) {
            break
        }

        if (grid[nextRow][nextCol] === '#') {
            currentDir = (currentDir + 1) % 4
            continue;
        }

        guardRow = nextRow
        guardCol = nextCol

        if (visited.get(`${guardRow},${guardCol}`)) {
            if (visited.get(`${guardRow},${guardCol}`).dir === currentDir) {
                isLoop = true
                break
            }
        }
        visited.set(`${guardRow},${guardCol}`, { x: guardRow, y: guardCol, dir: currentDir })
    }

    return isLoop
}

export function part2(input: string): number {
    const grid = input.trim().split('\n').map((row) => row.split(''))
    let positions = 0

    let guardRow = grid.findIndex((row) => row.includes('^'));
    let guardCol = grid[guardRow].indexOf('^');
    let visited = new Set([`${guardRow},${guardCol}`])
    let currentDir = 0

    //run(grid, guardRow, guardCol, visited, currentDir)

    return positions
}