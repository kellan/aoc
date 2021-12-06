import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let fish:  Map<number, number>  = lines[0]
	.split(',')
	.map(n => +n)
	.reduce( (acc, curr) => {
		return acc.set(curr, (acc.get(curr) ?? 0) +1)
	}, new Map<number, number>()
);

console.log(fish);
