

export function part1(input: string): number {
  const [rulesInput, updatesInput] = input.split('\n\n')
  const rules = rulesInput.split('\n').map(el => { return el.split('|') })

  const updates = updatesInput.split('\n').map(el => { return el.split(',') })

  const correctUpdates = updates.filter(update => { return rules.every(rule => isCorrect(rule, update)) })
  let results = correctUpdates.reduce((acc, curr) => acc + parseInt(curr[Math.floor(curr.length / 2)]), 0)

  return results
}

function isCorrect(rule: string[], update: string[]): boolean {
  if (update.includes(rule[0]) && update.includes(rule[1])) {
    if (update.indexOf(rule[0]) < update.indexOf(rule[1])) {
      return true
    } else {
      return false
    }
  }
  return true
}

export function part2(input: string): number {
  const [rulesInput, updatesInput] = input.split('\n\n')
  const rules = rulesInput.split('\n').map(el => { return el.split('|') })

  const updates = updatesInput.split('\n').map(el => { return el.split(',') })

  const incorrectUpdates = updates.filter(update => { return !rules.every(rule => isCorrect(rule, update)) })
  //console.log("incorrectUpdates", incorrectUpdates)

  const sortedUpdates = incorrectUpdates.map(update => {
    return update.toSorted((a, b) => {
      return rules.some(rule => {
        return rule[0] === a && rule[1] === b
      }) ? -1 : 1
    })
  })

  let results = sortedUpdates.reduce((acc, curr) => acc + parseInt(curr[Math.floor(curr.length / 2)]), 0)

  //console.log("sortedUpdates", sortedUpdates)

  return results
}