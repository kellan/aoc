import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');

let lines = input.split('\n').map( n => {
	return n.split('|').map( n => n.trim().split(" "))
});

let count = 0;

for (let i = 0; i < lines.length; i++) {
	let segments:string[] = lines[i][0];
	let output:string[] = lines[i][1];

	if (output) {
		output.forEach( n => {
			if ([2,3,4,7].includes(n.length)) {
				count += 1
			}	
		});
	}
}

console.log(count);