import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');
let lines = input.split('\n')

let open_close = new Map([
    [')', '('],
    [']', '['],
    ['}', '{'],
    ['>', '<']
]);

let close_open = new Map([
    ['(', ')'],
    ['[', ']'],
    ['{', '}'],
    ['<', '>']
]);

let scoring = new Map([
    [')', 1],
    [']', 2],
    ['}', 3],
    ['>', 4]
]);

lines = lines.filter(l => !is_corrupted(l));
//console.log(lines);

let autocomplete:string[][] = lines.map(l => balance(l));

let scores:number[] = new Array<number>();

for (let a of autocomplete) {
    let score = 0;
    for (let c of a) {
        let v = scoring.get(c) ?? 0;
        score *= 5
        score += v
    }
    scores.push(score)
}

scores.sort((a,b) => a < b ? 1 : -1);
console.log(scores[ Math.floor(scores.length/2) ] );

function balance(chunk:string): string[] {
    let open:string[] = new Array<string>();
    let balance:string[] = new Array<string>();

    for (let i = 0; i<chunk.length; i++) {
        if (['(', '[', '{', '<'].includes(chunk.charAt(i))) {
            open.push(chunk.charAt(i));
        } else {
            // presumably I don't need to check here as I've already filtered the corrupted ones
            open.pop();
        }
    }

    for (let c of open) {
        balance.push( close_open.get(c) ?? '' )
    }

    return balance.reverse();
}

function is_corrupted(chunk:string):boolean {
    let open:string[] = new Array<string>();

    let corrupted = false;

    for (let i = 0; i<chunk.length; i++) {
        if (['(', '[', '{', '<'].includes(chunk.charAt(i))) {
            open.push(chunk.charAt(i));
        } else {
            let current_open = open.pop();
            if (open_close.get(chunk.charAt(i)) != current_open) {
                return true;
            }
        }
    }

    return corrupted;
}