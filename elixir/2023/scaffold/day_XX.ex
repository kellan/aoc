defmodule Aoc2022.DayXX do
  @day "dayXX"

  def part1() do
  end

  def part2() do
  end


defmodule Mix.Tasks.DayXX do
  use Mix.Task

  def run([]) do
    solution = Aoc2022.DayXX.part1()
    IO.puts(solution)
  end

  def run([arg]) do
    case arg do
      "part1" ->
        solution = Aoc2022.DayXX.part1()
        IO.puts(solution)

      "part2" ->
        solution = Aoc2022.DayXX.part2()
        IO.puts(solution)

      _ ->
        IO.puts("Unknown argument: #{arg}")
        System.halt(1)
    end
  end
end
