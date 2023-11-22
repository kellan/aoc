defmodule Aoc2022.Day01 do
  @day "day01"

  def part1() do
    input = Aoc2022.process_input(@day)
  end

  def part2() do
    IO.puts("Hello")
  end
end

defmodule Mix.Tasks.Day01 do
  use Mix.Task

  def run(_) do
    solution = Aoc2022.Day01.part1()
    IO.puts(solution)
  end
end
