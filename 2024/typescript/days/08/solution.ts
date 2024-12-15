
type Antenna = {
    x: number,
    y: number,
    frequency: String
}

function parseInput(input: string): [string[][], number, number, Map<String, Antenna[]>] {
    const grid = input.trim().split('\n').map((row) => row.split(''))
    const height = grid.length
    const width = grid[0].length

    let antennas: Map<String, Antenna[]> = new Map()

    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            if (grid[y][x] !== '.') {
                let freq = grid[y][x]
                if (!antennas.has(freq)) {
                    antennas.set(freq, [])
                }
                antennas.get(freq)?.push({ x: x, y: y, frequency: freq })
            }
        }
    }

    return [grid, height, width, antennas]
}

export function part1(input: string): number {
    const [grid, height, width, antennas] = parseInput(input)

    let antinodes = new Map<String, [number, number]>()

    for (let [freq, positions] of antennas) {
        let pairs = generatePairs(positions)
        pairs.forEach((pair) => {
            let anodes = generateAntinodes1(pair, width, height)
            anodes.forEach((an) => {
                let key = `${an[0]},${an[1]}`
                antinodes.set(key, an)
            })
        })
    }
    //console.log(antinodes)
    return antinodes.size
}

function generateAntinodes1(pair: [Antenna, Antenna], width: number, height: number): [number, number][] {
    let antinodes: [number, number][] = []
    let [dx, dy] = [pair[1].x - pair[0].x, pair[1].y - pair[0].y]
    antinodes = [
        [pair[0].x + 2 * dx, pair[0].y + 2 * dy],
        [pair[1].x - 2 * dx, pair[1].y - 2 * dy]
    ]
    antinodes = antinodes.filter(([x, y]) => x >= 0 && y >= 0 && x < width && y < height)
    return antinodes
}

function generatePairs(arr: Antenna[]): [Antenna, Antenna][] {
    return arr.flatMap((x, i) => arr.slice(i + 1).map((y) => [x, y]));
}

export function part2(input: string): number {
    const [grid, height, width, antennas] = parseInput(input)

    let antinodes = new Map<String, [number, number]>()

    for (let [freq, positions] of antennas) {
        let pairs = generatePairs(positions)
        pairs.forEach((pair) => {
            let anodes = generateAntinodes2(pair, width, height)
            anodes.forEach((an) => {
                let key = `${an[0]},${an[1]}`
                antinodes.set(key, an)
            })
        })
    }

    return antinodes.size
}

function generateAntinodes2(pair: [Antenna, Antenna], width: number, height: number): [number, number][] {
    let antinodes: [number, number][] = []

    let [dx, dy] = [pair[1].x - pair[0].x, pair[1].y - pair[0].y]
    let [x, y] = [pair[0].x + dx, pair[0].y + dy]

    while (x >= 0 && y >= 0 && x < width && y < height) {
        antinodes.push([x, y])
        x += dx
        y += dy
    }

    [x, y] = [pair[0].x + dx, pair[0].y + dy]
    while (x >= 0 && y >= 0 && x < width && y < height) {
        antinodes.push([x, y])
        x -= dx
        y -= dy
    }


    return antinodes
}