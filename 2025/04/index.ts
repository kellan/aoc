import { lines } from "../util/lines";
import { runCli, sampleText } from "../util/cli";

import { 
    stringToGrid,
    getNeighborsCells,
    getCellsWithPredicate,
    setValueAt,
 } from "../util/grid";

import type { Grid, Coordinate} from "../util/grid";

export function part1(input: string) {
    let grid = stringToGrid(input)

    return moveableCells(grid).length
}
 
export function part2(input: string) {
    const grid = stringToGrid(input)
    let removed = 0;
    let cells = moveableCells(grid)
    while (cells.length > 0) {
        removed += cells.length
        for (const c of cells) {
            setValueAt(grid, c, ".")
        }
        cells = moveableCells(grid)
    }
    return removed
}
 
function moveableCells(grid: Grid<string>): Cell[] {
    return getCellsWithPredicate(grid, c => c.value == "@").
        filter(c => moveable(grid, c))
}

function moveable(grid: Grid<string>, coord: Coordinate): boolean {
    return getNeighborsCells(grid, coord).
        filter(c => c.value == "@").length < 4 ? true : false    
}

if (import.meta.main && process.env.NODE_ENV !== "test") {
  
  runCli({
      dir: import.meta.dir,
      part1,
      part2
  });
}


if (import.meta.main && process.env.NODE_ENV == "test") {
    const input = sampleText(import.meta.dir)
    let grid = stringToGrid(input)

    let cells = getCellsWithPredicate(grid, c => c.value == "@").
        filter(c => moveable(grid, c))

    test("day 04 sample", () => {
        expect(cells.length).toEqual(13)
})

}