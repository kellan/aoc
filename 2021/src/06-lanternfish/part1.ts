import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let fish: string[] = lines[0].split(',') || []

let fishDays: Map<string, number> = fish.reduce( (acc, curr) => {
	return acc.set(curr, (acc.get(curr) ?? 0) +1)
}, new Map<string, number>());

console.log(fishDays);
