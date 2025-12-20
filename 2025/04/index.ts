import { lines } from "../util/lines";
import { runCli, sampleText } from "../util/cli";

import { 
    stringToGrid,
    getNeighborsWithValues,
    getCellsWithPredicate,
 } from "../util/grid";

import type { Grid, Coordinate} from "../util/grid";

export function part1(input: string) {
    let grid = stringToGrid(input)

    let cells = getCellsWithPredicate(grid, c => c.value == "@").
        filter(c => moveable(grid, c))

    return cells.length
}
 
export function part2(input: string) {
}
 
function moveable(grid: Grid<string>, coord: Coordinate): boolean {
    return getNeighborsWithValues(grid, coord).
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