defmodule Aoc2022.Day01 do
  @day "day01"

  def part1() do
    Aoc2022.process_input(@day)
    |> String.split("\n\n", trim: true)
    |> Enum.map(&sum_lines/1)
    |> Enum.max()
  end

  def sum_lines(sub_input) do
    sub_input
    |> String.split("\n", trim: true)
    |> Enum.map(&String.to_integer/1)
    |> Enum.sum()
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
