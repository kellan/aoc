export function part1(input: string): number {

    let diskmap = input.split('').map((char, index) => {
        return index % 2 === 0 
            ? Array(parseInt(char)).fill(String(index/2))
            : Array(parseInt(char)).fill('.')
    }).filter(row => row.length > 0).flat()

    let i = 0
    let j = diskmap.length - 1

    while (i < j) {
        if (diskmap[i] !== '.') {
            i++
            continue
        }
        while (diskmap[j] === '.') { 
            j--
        }
        let free = diskmap[i]
        diskmap[i]= diskmap[j]
        diskmap[j] = free
 
        i++
        j--
    }

    //console.log(diskmap)

    let checksum = diskmap.reduce((acc, val, i) => {
        return val === '.' 
            ? acc 
            : acc + i*parseInt(val)
    }, 0)

    return checksum
}


export function part2(input: string): number {
    return 2
}