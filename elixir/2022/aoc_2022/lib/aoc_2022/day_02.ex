defmodule Aoc2022.Day02 do
  @day "day02"
  @lose 0
  @draw 3
  @win 6
  @rock 1
  @paper 2
  @scissors 3

  def part1() do
    Aoc2022.process_input(@day)
    |> String.split("\n", trim: true)
    |> Enum.map(&score_line1/1)
    |> Enum.sum()
  end

  def score_line1(line) do
    case line do
      "A X" -> @draw + @rock
      "A Y" -> @win + @paper
      "A Z" -> @lose + @scissors
      "B X" -> @lose + @rock
      "B Y" -> @draw + @paper
      "B Z" -> @win + @scissors
      "C X" -> @win + @rock
      "C Y" -> @lose + @paper
      "C Z" -> @draw + @scissors
      _ -> 0
    end
  end

  def part2() do
    Aoc2022.process_input(@day)
    |> String.split("\n", trim: true)
    |> Enum.map(&score_line2/1)
    |> Enum.sum()
  end

  def score_line2(line) do
    case line do
      # lose
      "A X" -> @lose + @scissors
      "B X" -> @lose + @rock
      "C X" -> @lose + @paper
      # draw
      "A Y" -> @draw + @rock
      "B Y" -> @draw + @paper
      "C Y" -> @draw + @scissors
      # win
      "A Z" -> @win + @paper
      "B Z" -> @win + @scissors
      "C Z" -> @win + @rock
      _ -> 0
    end
  end
end

defmodule Mix.Tasks.Day02 do
  use Mix.Task

  def run([]) do
    solution = Aoc2022.Day02.part1()
    IO.puts(solution)
  end

  def run([arg]) do
    case arg do
      "part1" ->
        solution = Aoc2022.Day02.part1()
        IO.puts(solution)

      "part2" ->
        solution = Aoc2022.Day02.part2()
        IO.puts(solution)

      _ ->
        IO.puts("Unknown argument: #{arg}")
        System.halt(1)
    end
  end
end
