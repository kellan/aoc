import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let [height, width] = [lines.length, lines[0].length];
let grid:number[][] = new Array<Array<number>>(height);

for (let y=0; y<height;y++) {
    grid[y] = new Array<number>(width);
    for (let x=0; x<width; x++) {
        grid[y][x] = parseInt(lines[y].charAt(x))
    }
}

//console.log(grid);

let steps = 1000;

for(let i=0; i<steps; i++) {

    // all octopus increases by 1
    for (let y=0; y<height;y++) {
        for (let x=0;x<width; x++) {
            grid[y][x] += 1;
        }
    }

    let if_flashed = true;
    let flashed_dumbo:Map<string,boolean> = new Map<string,boolean>();

    let flash_count = 0;

    while(if_flashed) {
        if_flashed = false
        
        for (let y=0; y<height;y++) {
            for (let x=0;x<width; x++) {

                // hasn't flashed and 9+ energy
                if (!flashed_dumbo.has( [x,y].join(',') ) && grid[y][x] > 9) {

                    if_flashed = true;                    
                    flashed_dumbo.set([x,y].join(','), true);
                    flash_count += 1;

                    // increase each neighbor
                    neighbors(grid, x, y).forEach(point => {
                        grid[point[1]][point[0]] += 1;
                    });
                }
            }
        }

        if (flash_count == 100) {
            console.log("Step: ", i+1);
            process.exit()
        }
    }

    flashed_dumbo.forEach( (v,k) => {
        let [x,y] = k.split(',').map( n => +n) || [0,0];
        grid[y][x] = 0;
    });

}

//console.log("flash counts", flash_count);


function neighbors(grid:number[][], x:number, y:number): number[][] {
    let neighbors:number[][] = new Array<Array<number>>();

    let deltas:number[][] = [ [-1,-1],[0,-1],[1,-1],[-1,0],[1,0],[-1,1],[0,1],[1,1] ];
    
    deltas.forEach(delta => {
        neighbors.push( [x+delta[0], y+delta[1]] );
    });

    return neighbors.filter(point => valid_point(grid, point[0], point[1]) );
}

function valid_point(grid:number[][], x:number, y:number): boolean {
    if (0 <= x && x < grid[0].length &&
        0 <= y && y < grid.length) {
        return true;
    } else {
        return false;
    }
}

//     deltas.forEach(delta => {
//         neighbors.push([x+delta[0], y+delta[1]]);
//     });

//     return neighbors.filter(point => {
//         if (0 <= point[0] &&
//             point[0] < grid[0].length &&
//             0 <= point[1] &&
//             point[1] < grid.length) {
//             return true;
//         } else {
//             return false;
//         }
//     });
// }