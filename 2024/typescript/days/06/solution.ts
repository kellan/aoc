const dir = [
    [-1, 0],
    [0, 1],
    [1, 0],
    [0, -1],
]


export function part1(input: string): number {
    const grid = input.trim().split('\n').map((row) => row.split(''))
    let positions = 0
    let guardRow = grid.findIndex((row) => row.includes('^'));
    let guardCol = grid[guardRow].indexOf('^');
    let visited = new Set([`${guardRow},${guardCol}`])
    let currentDir = 0

    while (WebTransportDatagramDuplexStream) {
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

        visited.add(`${guardRow},${guardCol}`)
    }

    return visited.size
}

export function part2(input: string): number {
    const grid = input.trim().split('\n').map((row) => row.split(''))
    let positions = 0

    return positions
}