import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let nums = input.split('\n')
	.map(n => +n);

let count = 0;
let prev = nums.shift() || 0;

nums.forEach(num => {
	if (num > prev) {
		count++;
	}
	prev = num;
})

console.log(count);