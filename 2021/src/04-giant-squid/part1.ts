import * as fs from 'fs';

interface Board {
	numbers: number[];
	where: Map<number, [number, number]>;

	mrow: number[];
	mcol: number[];
}

function parseBoard(input: string): Board {
	let numbers: number[] = new Array();
	let where: Map<number, [number, number]> = new Map();
	let mrow: number[] = new Array();
	let mcol: number[] = new Array();
	
	let rows = input.split('\n');
	rows.forEach( (row, y:number) => {
		let nums = row.split(' ');
		nums.forEach( (n, x:number) => {
			where.set(+n, [x,y]);
			numbers.push(+n);
		});
	});

	return {numbers: numbers, where: where, mrow: mrow, mcol: mcol};
}


let boardString = `22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19`;


 //console.log(boardString);

let b: Board = parseBoard(boardString);

console.log(b);