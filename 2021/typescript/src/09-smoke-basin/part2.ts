import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let [height, width] = [lines.length, lines[0].length];

let map:number[][] = new Array<Array<number>>(height);

for (let y=0; y<height;y++) {
    map[y] = new Array<number>(width);
    for (let x=0; x<width; x++) {
        map[y][x] = parseInt(lines[y].charAt(x))
    }
}

let low_points = find_mins(map);
//console.log(low_points);

let basins: string[][] = new Array<Array<string>>();

for (let point of low_points) {
    let basin = find_basin(map, point);
    basins.push(basin);
}

//console.log(basins);

basins.sort((a,b) => a.length < b.length ? 1 : -1);

let answer = basins.slice(1,3).reduce( (p,c) => p*c.length, basins[0].length);
console.log(answer);

function find_basin(map:number[][], point:string): string[] {
    let basin:string[] = new Array<string>();
    let visited:Map<string,boolean> = new Map<string,boolean>();
    let queue:string[] = new Array<string>();

    queue.push(point);
    while (queue.length > 0) {
        let p_i:string = queue.pop() || '';

        if (visited.has(p_i)) {
            continue;
        } else {
            visited.set(p_i, true);
            let [x_i,y_i] = p_i.split(',').map(n => +n);

            if (map[y_i][x_i] != 9) {
                basin.push(p_i);
                let new_points = surrounding(map, x_i, y_i).filter( p => !visited.has(p));
                queue = queue.concat(new_points);
            }
        }
    }
    return basin;
}



function find_mins(map:number[][]): string[] {
    let low_points:string[] = new Array<string>();

    for (let y=0; y<height;y++) {
        for (let x=0;x<width; x++) {
            if (is_min(map, x, y)) {
                low_points.push([x,y].join(','))
            }
        }
    }

    return low_points;
}


function is_min(map:number[][], x:number, y:number): boolean {
    let surrounds:string[] = surrounding(map, x, y);
    
    for (let delta of surrounds) {
        let [x_i,y_i] = delta.split(',').map(n => +n);
        if (map[y][x] == 9 || map[y_i][x_i] < map[y][x]) {
            return false;
        }
    }
    return true;
}

function surrounding(map:number[][], x:number, y:number): string[] {
    let surrounds:number[][] = new Array<Array<number>>();

    [[-1,0], [0, -1], [1, 0], [0, 1]].forEach(delta => {
        surrounds.push([x+delta[0], y+delta[1]]);
    });

    let points:string[] = surrounds.filter(point => {
        if (0 <= point[0] &&
            point[0] < map[0].length &&
            0 <= point[1] &&
            point[1] < map.length) {
            return true;
        } else {
            return false;
        }
    }).map(p => p.join(','));

    return points;
}