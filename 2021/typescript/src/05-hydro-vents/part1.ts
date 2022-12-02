import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let points = new Map();

for (let line of lines) {
	let found = line.match(/(\d+),(\d+) -> (\d+),(\d+)/) || [];
	let [x1, y1, x2, y2]= [found[1],found[2],found[3],found[4]].map(n => +n);

	if (x1 != x2 && y1 != y2) {
		continue;
	}

	let dist = Math.max(Math.abs(x2-x1), Math.abs(y2-y1))

	let dx = 0;

	if (x2 > x1) {
		dx = 1;
	} else if (x1 > x2) {
		dx = -1;
	}

	let dy = 0;
	
	if (y2 > y1) {
		dy = 1;
	} else if (y1 > y2) {
		dy = -1;
	}

	for (let i = 0; i <= dist; i++) {
		let x = x1 + i*dx;
		let y = y1 + i*dy;

		points.set(`${x},${y}`, (points.get(`${x},${y}`) ?? 0) + 1)
	}
}

let intersect = 0
points.forEach( (val, key) => {
	if (val > 1) {
		intersect += 1;
		console.log(key);
	}
});

console.log(intersect);
