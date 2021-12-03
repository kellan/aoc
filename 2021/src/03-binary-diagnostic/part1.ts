import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let sums = new Array<number>(lines[0].length).fill(0)

lines.forEach(line => {
	for(let i = 0; i < line.length; i++) { 	
		sums[i] += +line[i];
	}
});

let half = lines.length / 2;

let gamma = sums.reduce((acc, curr, i) => {
	if (curr >= half) {
		acc += Math.pow(2, sums.length - i-1)
	}
	return acc;
},0);

let epsilon = sums.reduce((acc, curr, i) => {
	if (curr < half) {
		acc += Math.pow(2, sums.length - i-1)
	}
	return acc;
},0);




// console.log("gamma:" + gamma);
// console.log("epsilon:" + epsilon);

// console.log(gamma*epsilon);