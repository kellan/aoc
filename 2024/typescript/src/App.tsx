import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

const solutions = import.meta.glob('/days/*/solution.ts')
const testInputs = import.meta.glob('/days/*/test.txt', { as: 'raw' })
const realInputs = import.meta.glob('/days/*/input.txt', { as: 'raw' })

function App() {

  const [results, setResults] = useState<{
    day: string,
    part1Test: number,
    part2Test: number,
    part1: number,
    part2: number
  }>({
    day: '',
    part1Test: 0,
    part2Test: 0,
    part1: 0,
    part2: 0
  })

  const runSolution = async (day: string) => {
    if (solutions[`/days/${day}/solution.ts`]) {
      const module = await solutions[`/days/${day}/solution.ts`]()
      const testData = await testInputs[`/days/${day}/test.txt`]()
      const realData = await realInputs[`/days/${day}/input.txt`]()

      setResults({
        day: day,
        part1Test: module.part1(testData),
        part2Test: module.part2(testData),
        part1: module.part1(realData),
        part2: module.part2(realData)
      })
    }
  }

  runSolution('01')

  return (
    <div className="app" >
      <nav className="sidebar">
        <ul>
          <li><a href="#home">Foo</a></li>
          <li><a href="#dashboard">Dashboard</a></li>
          <li><a href="#settings">Settings</a></li>
        </ul>
      </nav>

      <main className="main-content">
        <header>
          <h1>Day {results.day}</h1>
        </header>
        <div className="content">
          <table border="1" cellPadding="10">
            <thead>
              <tr>
                <th></th><th>Part 1</th><th>Part 2</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>Test</td><td>{results.part1Test}</td><td>{results.part2Test}</td>
              </tr>
              <tr>
                <td>Real</td><td></td><td></td>
              </tr>
            </tbody>
          </table>
        </div>
      </main >
    </div >
  )
}

export default App
