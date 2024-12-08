export function part1(input: string): number {
  const matches = [...input.matchAll(/mul\((\d{1,3}),(\d{1,3})\)/g)]
  //console.log(matches)

  const results = matches.map(match => {
    //console.log(`match[1] ${match[1]} match[2] ${match[2]}`)
    return parseInt(match[1]) * parseInt(match[2])
  }).reduce((acc, curr) => acc + curr, 0)

  return results
}

export function part2(input: string): number {
  const matches = [...input.matchAll(/mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)/g)]
  let enabled = true
  let sum = 0
  matches.forEach(match => {
    switch (match[0]) {
      case 'do()':
        //console.log('do')
        enabled = true
        break
      case 'don\'t()':
        //console.log('don\'t')
        enabled = false
        break
      default:
        //console.log(`mul(${match[1]}, ${match[2]})`)
        if (enabled) {
          sum += parseInt(match[1]) * parseInt(match[2])
        }
    }
  })

  return sum
}