import * as fs from 'fs';


function getSums(lines: number[][]): number[] {
	let sums = new Array<number>(lines[0].length).fill(0)

	lines.forEach(line => {
		for(let i = 0; i < line.length; i++) { 	
			sums[i] += line[i];
		}
	});
	return sums;
}

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')
	.map(line => line.split('').map((n => +n)))

let oxygen = lines;

for (let i = 0; i < lines[0].length; i++) {
	let sums = getSums(oxygen);
	let half = oxygen.length / 2;
	let target = sums[i] >= half ? 1 :0;
	
	oxygen = oxygen.filter( (line) => {
		return line[i] == target
	});

	if (oxygen.length == 1) {
		break;
	}
}

let oxygenDec= oxygen[0].reduce((acc, curr) => (acc << 1) | curr)


let co2 = lines;

for (let i = 0; i < lines[0].length; i++) {
	let sums = getSums(co2);
	let half = co2.length / 2;
	let target = sums[i] >= half ? 0 :1;
	
	co2 = co2.filter( (line) => {
		return line[i] == target
	});

	if (co2.length == 1) {
		break;
	}
}

let co2Dec= co2[0].reduce((acc, curr) => (acc << 1) | curr)

console.log(oxygenDec, co2Dec);
console.log(oxygenDec*co2Dec);

