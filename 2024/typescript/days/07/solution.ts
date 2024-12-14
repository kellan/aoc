export function part1(input: string): number {
    const calibrations = input.trim().split('\n').map((row) => row.split(/: | /).map(Number))

    let results = 0

    for (let calibration of calibrations) {
        let [target, ...nums] = calibration
        let possible_values = permutate(nums, ['*', '+'])
        if (possible_values.includes(target)) {
            results += target
        }
    }

    return results
}

function permutate(nums: number[], operators: string[]): number[] {
    let accumulations: number[] = [nums.shift()]
    //console.log(accumulations, nums)

    for (let i = 0; i < nums.length; i++) {
        let new_acc = []
        for (let j = 0; j < accumulations.length; j++) {
            for (let op of operators) {
                if (op === '*') {
                    new_acc.push(accumulations[j] * nums[i])
                } else if (op === '+') {
                    new_acc.push(accumulations[j] + nums[i])
                } else if (op === 'concat') {
                    new_acc.push(parseInt("" + accumulations[j] + nums[i]))
                }
            }
        }
        accumulations = new_acc
    }
    return accumulations
}


export function part2(input: string): number {
    const calibrations = input.trim().split('\n').map((row) => row.split(/: | /).map(Number))
    //let calibrations = [[156, 15, 6]]

    let results = 0

    for (let calibration of calibrations) {
        let [target, ...nums] = calibration
        let possible_values = permutate(nums, ['*', '+', 'concat'])

        if (possible_values.includes(target)) {
            results += target
        }
    }

    return results
}
