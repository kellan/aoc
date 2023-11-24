defmodule Aoc2022.Day04 do
  @day "day04"

  def part1() do
    Aoc2022.process_input(@day, :split)
    |> Enum.map(&contains?/1)
    |> Enum.sum()
  end

  def part2() do
    Aoc2022.process_input(@day, :split)
    |> Enum.map(&overlaps?/1)
    |> Enum.sum()
  end

  def contains?(pair) do
    [[l1, l2], [r1, r2]] = to_pair(pair)

    if (l1 <= r1 && l2 >= r2) or (r1 <= l1 && r2 >= l2) do
      1
    else
      0
    end
  end

  def overlaps?(pair) do
    [[l1, l2], [r1, r2]] = to_pair(pair)

    if l2 >= r1 && l1 <= r2 do
      1
    else
      0
    end
  end

  def to_pair(str) do
    [left, right] = String.split(str, ",")

    [
      String.split(left, "-") |> Enum.map(&String.to_integer/1),
      String.split(right, "-") |> Enum.map(&String.to_integer/1)
    ]
  end
end

defmodule Mix.Tasks.Day04 do
  use Mix.Task

  def run([]) do
    run(["part1"])
  end

  def run([arg]) do
    case arg do
      "part1" ->
        solution = Aoc2022.Day04.part1()

        IO.puts(solution)

      "part2" ->
        solution = Aoc2022.Day04.part2()
        IO.puts(solution)

      _ ->
        IO.puts("Unknown argument: #{arg}")
        System.halt(1)
    end
  end
end
