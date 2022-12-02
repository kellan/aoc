import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let [height, width] = [lines.length, lines[0].length];
// console.log(height, width);

let map:number[][] = new Array<Array<number>>(height);

for (let y=0; y<height;y++) {
	map[y] = new Array<number>(width);
	for (let x=0; x<width; x++) {
		map[y][x] = parseInt(lines[y].charAt(x))
	}
}


let low_points:number[] = new Array<number>();

for (let y=0; y<height;y++) {
	for (let x=0;x<width; x++) {
		if (is_min(map, x, y)) {
//			console.log("found min: ", x, y, map[y][x]);
			low_points.push(map[y][x]);
		}
	}
}

let sum = low_points.reduce((a,b) => a+b+1, 0);

console.log(sum);

function is_min(map:number[][], x:number, y:number): boolean {
	let surrounds:number[][] = surrounding(map, x, y);
	
	for (let delta of surrounds) {
		if (map[y][x] == 9 || map[delta[1]][delta[0]] < map[y][x]) {
			return false;
		}
	}
	return true;
}

function surrounding(map:number[][], x:number, y:number): number[][] {
	let surrounds:number[][] = new Array<Array<number>>();
	[[-1,0], [0, -1], [1, 0], [0, 1]].forEach(delta => {
		surrounds.push([x+delta[0], y+delta[1]]);
	});

	return surrounds.filter(point => {
		if (0 <= point[0] &&
			point[0] < map[0].length &&
			0 <= point[1] &&
			point[1] < map.length) {
			return true;
		} else {
			return false;
		}
	});

}