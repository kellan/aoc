import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let x = 0;
let y = 0;

lines.forEach(line => {
	let [cmd, steps] = line.split(" ")
	if (cmd == "forward") {
		x += +steps;
	} else if (cmd == "down") {
		y += +steps;
	} else if (cmd == "up") {
		y -= +steps;
	}
});

console.log(x*y);
