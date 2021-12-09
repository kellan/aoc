import * as fs from 'fs';



function triangle(n: number): number {
    return (n*(n+1)/2)
}

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n');

let positions: number[] = lines[0]
    .split(',')
    .map(n => +n)
    .sort((a,b) => a-b);

//console.log(positions);

let [start,end] = [positions[0], positions[positions.length-1]]; 

//console.log(start, end);

let best_pos = 0;
let best_score = Number.MAX_SAFE_INTEGER;

for (let i = start; i <= end; i++) {
    let score = 0;
    for (let j = 0; j < positions.length; j++) {
        score += triangle(Math.abs(i - positions[j]));
    }
    if (score < best_score) {
        best_pos = i;
        best_score = score;
    }
}

console.log(best_pos, best_score);

//console.log(triangle(4), triangle(5), triangle(10), triangle(11));
