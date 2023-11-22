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
    Aoc2022.process_input(@day)
    |> String.split("\n\n", trim: true)
    |> Enum.map(&sum_lines/1)
    |> Enum.sort()
    |> Enum.take(-3)
    |> Enum.sum()
  end
end

defmodule Mix.Tasks.Day01 do
  use Mix.Task

  def run([]) do
    solution = Aoc2022.Day01.part1()
    IO.puts(solution)
  end

  def run([arg]) do
    case arg do
      "part1" ->
        solution = Aoc2022.Day01.part1()
        IO.puts(solution)

      "part2" ->
        solution = Aoc2022.Day01.part2()
        IO.puts(solution)

      _ ->
        IO.puts("Unknown argument: #{arg}")
        System.halt(1)
    end
  end
end
