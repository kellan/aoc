import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let nums = input.split('\n')
	.map(n => +n);

let count = 0;
let prevSum = nums.slice(0,3).reduce((a,b) => a+b);

for(let i = 1; i < nums.length - 2; i++) {
	let currSum = nums.slice(i, i+3).reduce((a,b) => a+b)
	if (currSum > prevSum) {
		count++;
	}
	prevSum = currSum;
}

console.log(count);