import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let x = 0;
let y = 0;

for (let line of lines) {
	let found = line.match(/(\d+),(\d+) -> (\d+),(\d+)/) || [];
	let [x1, x2, y1, y2]= [found[1],found[2],found[3],found[4]];

	if (x1 != x2 && y1 != y2) {
		continue;
	}

	console.log(x1, x2, y1, y2);
}
