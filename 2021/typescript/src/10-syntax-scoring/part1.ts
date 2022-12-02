import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let open_close = new Map([
    [')', '('],
    [']', '['],
    ['}', '{'],
    ['>', '<']
]);

let scoring = new Map([
    [')', 3],
    [']', 57],
    ['}', 1197],
    ['>', 25137]
]);

let corrupted:string[] = lines.map(l => parse_line(l)).filter(x => x !== undefined).map(n => n as string);
//console.log(corrupted);

// let corrupted_score:number = corrupted.reduce( (p,c) => {
//     let n:number = scoring.get(c) ?? 0;
//     return p+c;
// }, 0);
// console.log(corrupted_score);

let score = 0;

for (let c of corrupted) {
    score += scoring.get(c) ?? 0;
}

console.log(score);


function parse_line(chunk:string) {
    let open:string[] = new Array<string>();

    for (let i = 0; i<chunk.length; i++) {
        if (['(', '[', '{', '<'].includes(chunk.charAt(i))) {
            open.push(chunk.charAt(i));
        } else {
            let current_open = open.pop();
            if (open_close.get(chunk.charAt(i)) != current_open) {
                return chunk.charAt(i);
            }
        }
    }
}