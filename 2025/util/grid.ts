export type Grid<T> = T[][];
export type Coordinate = { row: number; col: number };
export type Cell<T> = Coordinate & { value: T } 

export const stringToGrid = (input: string): Grid<string> => {
    return input
        .trimEnd()
        .split("\n")
        .map(line => line.split(""));
}

export const inBounds = <T>(g: Grid<T>, c: Coordinate): boolean =>
  c.row >= 0 && c.row < g.length && c.col >= 0 && c.col < g[0].length;

export const getAt = <T>(g: Grid<T>, c: Coordinate): T | undefined =>
  inBounds(g, c) ? g[c.row][c.col] : undefined;


export const getCoordinates = <T>(g: Grid<T>): Coordinate[] => {
    const result: Coordinate[] = [];
    for (let row = 0; row < g.length; row++) {
        for (let col = 0; col < g[row].length; col++) {
            result.push({ row, col });
        }
  }
  return result;
}

export const getCells = <T>(g: Grid<T>): Cell[] => {
    const result: Cell[] = []
    for (let row = 0; row < g.length; row++) {
        for (let col = 0; col < g[row].length; col++) {
            result.push({ row, col, value: g[row][col] });
        }
    }
    return result
}

export const getNeighbors = <T>(g: Grid<T>, c: Coordinate): Coordinate[] => {
  const candidates: Coordinate[] = [
    { row: c.row - 1, col: c.col - 1 },
    { row: c.row - 1, col: c.col },
    { row: c.row - 1, col: c.col + 1 },
    { row: c.row, col: c.col - 1 },
    { row: c.row, col: c.col + 1 },
    { row: c.row + 1, col: c.col - 1 },
    { row: c.row + 1, col: c.col },
    { row: c.row + 1, col: c.col + 1 },
  ];

  return candidates.filter((n) => inBounds(g, n));
};

export const getNeighborsWithValues = <T>(g: Grid<T>, c: Coordinate): Cell<T>[] => {
    return getNeighbors(g, c).map(n => ({
        ...n,
        value: g[n.row][n.col]
    }))
}
