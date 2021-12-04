import * as fs from 'fs';

class Bingo {
    draw: number[];
    boards: Board[];
    winners: Board[];

    constructor(input: string) {
        let blocks = input.split('\n\n');
        this.draw = blocks.shift()?.split(',').map(n => +n) || [];
        this.boards = blocks.map((b) => new Board(b));
        this.winners = [];
    }

    play() {
        for (let num of this.draw) {
            for (let board of this.boards) {
                if (board.isWinner()) {
                    continue;
                }

                board.mark(num);

                if (board.isWinner()) {
                    this.winners.push(board);
                }
            }
        }
    }
}

class Board {
    unmarkedNumbers: number[];
    lastMarked: number;
    where: Map<number, [number, number]>;
    mrow: number[];
    mcol: number[];

    constructor(input: string) {
        
        this.lastMarked = -1;
        this.mrow = new Array(5);
        this.mcol = new Array(5);
        this.unmarkedNumbers = new Array();
        this.where = new Map();
        
        let rows = input.split('\n');
        rows.forEach( (row, y:number) => {
            let nums = row.trim().split(/\s+/); // make sure to split on more than 1 space because space padded input
            
            nums.forEach( (n, x:number) => {
                this.where.set(+n, [x,y]);
                this.unmarkedNumbers.push(+n);
            });
        });
    }

    mark(num: number) {
        if (this.where.has(num)) {
            // just here to make TS happy, i would expect to be able to do
            // let [x,y] = this.where.get(num)
            let xy = this.where.get(num);
            if (xy) {
                this.mrow[xy[0]] = this.mrow[xy[0]] ? this.mrow[xy[0]]+1 : 1;
                this.mcol[xy[1]] = this.mcol[xy[1]] ? this.mcol[xy[1]]+1 : 1;
                let mIndex = this.unmarkedNumbers.indexOf(num);
                this.unmarkedNumbers.splice(mIndex, 1);
                this.lastMarked = num;
            }
        }
    }

    isWinner(): boolean {
        if (this.mrow.indexOf(5) > -1 || this.mcol.indexOf(5) > -1) {
            return true;
        } else {
            return false;
        }
    }

    score(): number {
        let sum = this.unmarkedNumbers.reduce((a,b) => a+b, 0);
        return sum*this.lastMarked;
    }
}


let input = fs.readFileSync(process.stdin.fd, 'utf-8');

let game = new Bingo(input);

game.play();

let firstWin = game.winners[0];
let lastWin = game.winners[ game.winners.length - 1];

console.log("first winner: " + firstWin.score());
console.log("last winner: " + lastWin.score());