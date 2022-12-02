import * as fs from 'fs';

interface Game {
    draw: number[];
    boards: Board[];
}

interface Board {
    unmarkedNumbers: number[];
    lastMarked: number;
    where: Map<number, [number, number]>;

    mrow: number[];
    mcol: number[];
}

function parseBoard(input: string): Board {
    let unmarkedNumbers: number[] = new Array();
    let where: Map<number, [number, number]> = new Map();
    let mrow: number[] = new Array();
    let mcol: number[] = new Array();
    
    let rows = input.split('\n');
    rows.forEach( (row, y:number) => {
        let nums = row.trim().split(/\s+/); // make sure to split on more than 1 space because space padded input
        nums.forEach( (n, x:number) => {
            where.set(+n, [x,y]);
            unmarkedNumbers.push(+n);
        });
    });

    return {unmarkedNumbers: unmarkedNumbers, lastMarked: -1, where: where, mrow: mrow, mcol: mcol};
}

function parseGame(input: string): Game {
    let blocks = input.split('\n\n');
    let draw: number[] = blocks.shift()?.split(',').map(n => +n) || [];

    let boards: Board[] = blocks.map((b) => {
        return parseBoard(b);
    });

    return {draw: draw, boards: boards};
}

function playGame(game: Game): Board | undefined {
    for (let num of game.draw) {
        for (let board of game.boards) {
            if (board.where.has(num)) {
                // just here to make TS happy, i would expect to be able to do
                // let [x,y] = board.where.get(num);
                let [x,y] = board.where.get(num) || [];

                if (x) {
                    board.mrow[x] = board.mrow[x] ? board.mrow[x]+1 : 1;
                }
                if (y) {
                    board.mcol[y] = board.mcol[y] ? board.mcol[y]+1 : 1;
                }
                
                let mIndex = board.unmarkedNumbers.indexOf(num);
                board.unmarkedNumbers.splice(mIndex, 1);

                board.lastMarked = num;

                // if we've marked 5 in a row in a row or column we have a winner.
                if (board.mrow.indexOf(5) > -1 || board.mcol.indexOf(5) > -1) {
                    return board;
                }
            }
        }
    }
}

let input = fs.readFileSync(process.stdin.fd, 'utf-8');

let game = parseGame(input);

let firstWin = playGame(game);

if (firstWin) {
    let unmarkedSum = firstWin.unmarkedNumbers.reduce((a,b) => a+b, 0);
    console.log(unmarkedSum*firstWin.lastMarked);
}
