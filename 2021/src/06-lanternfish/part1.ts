import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n');

let fish:  Map<number, number>  = lines[0]
	.split(',')
	.map(n => +n)
	.reduce( (acc, curr) => {
		return acc.set(curr, (acc.get(curr) ?? 0) +1)
	}, new Map<number, number>()
);


for (let d = 0; d < 256; d++) {
	// one day
	let newFish: Map<number, number> = new Map();
	for (let i = 8; i > 0; i--) {
		if (fish.get(i)) {
			newFish.set(i-1, fish.get(i) ?? 0);
		}
	}
	if(fish.get(0)) {
		newFish.set(8, fish.get(0) ?? 0);

		newFish.set(6, 
			(newFish.get(6) ?? 0) + (fish.get(0) ?? 0)
		);
	}
	fish = newFish;
}

// why doesn't this work?
//console.log(fish.values.reduce((acc, val) => acc+val));

let sum = 0;
fish.forEach( (val, key) => {
	sum += val;
});

console.log(sum);