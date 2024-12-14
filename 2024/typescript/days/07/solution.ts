export function part1(input: string): number {
    const calibrations = input.trim().split('\n').map((row) => row.split(/: | /).map(Number))

    let results = 0

    for (let calibration of calibrations) {
        let [target, ...nums] = calibration
        let possible_values = permutate(nums)
        if (possible_values.includes(target)) {
            results += target
        }
    }

    return results
}

function permutate(nums: number[]): number[] {
    let accumulations: number[] = [nums.shift()]

    for (let i = 0; i < nums.length; i++) {
        let new_acc = []
        for (let j = 0; j < accumulations.length; j++) {
            new_acc.push(accumulations[j] * nums[i])
            new_acc.push(accumulations[j] + nums[i])
        }
        accumulations = new_acc
    }
    return accumulations
}


export function part2(input: string): number {
    return 2
}
