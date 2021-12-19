import * as fs from 'fs';

let input = fs.readFileSync(process.stdin.fd, 'utf-8');

let lines = input.split('\n').map( n => {
    return n.split('|').map( n => n.trim().split(" ") )
});

let count = 0;

for (let i = 0; i < lines.length; i++) {
    let segments:string[] = lines[i][0];

    let mapping = generate_mapping(segments);

    let output:string[] = lines[i][1];
    
    let sequence = "";

    if (output) {
        output.forEach( n => {
            n = n.split('').sort().join('');

            if (mapping.has(n)) {
                sequence = sequence.concat(mapping.get(n) || "");
            }
        });
    }
    
    count += +sequence;
}

//console.log(count);
console.log(count);


function generate_mapping(segments:string[]): Map<string,string> {
    

    let one = segments.find(s => s.length == 2) || '';
    let seven = segments.find(s => s.length == 3) || '';
    let four = segments.find(s => s.length == 4) || '';
    let eight = segments.find(s => s.length == 7) || '';

    // let two = '';
    // let three = '';
    // let five = '';

    // if (s.length == 5) {

    // }

    let zero = '';
    let six = '';
    let nine = '';

    let two = '';
    let three = '';
    let five = '';

    segments.forEach( s => {
        if (s.length == 6) {
            if (has_subset(s, four)) {
                nine = s;
            } else if (!has_subset(s, four) && has_subset(s, seven)) {
                // this pattern discovered by Anwen
                zero = s;
            } else {
                six = s;
            }
        }
    });

    segments.forEach( s => {
        if (s.length == 5) {
            if (has_subset(s, one)) {
                three = s;
            } else if (has_subset(six, s)) {
                five = s;
            } else {
                two = s;
            }
        }
    });
    
    let mapping:Map<string,string> = new Map([
            [zero.split('').sort().join(''), "0"],
            [one.split('').sort().join(''), "1"],
            [two.split('').sort().join(''), "2"],
            [three.split('').sort().join(''), "3"],
            [four.split('').sort().join(''), "4"],
            [five.split('').sort().join(''), "5"],
            [six.split('').sort().join(''), "6"],
            [seven.split('').sort().join(''), "7"],
            [eight.split('').sort().join(''), "8"],
            [nine.split('').sort().join(''), "9"],
    ]);

    //console.log(mapping);
    return mapping;

}

function has_subset(segmentA: string, segmentB: string): boolean {

    //console.log(segmentA, segmentB);

    return segmentB.split('').every(n => {
        return segmentA.split('').includes(n)
    });

}