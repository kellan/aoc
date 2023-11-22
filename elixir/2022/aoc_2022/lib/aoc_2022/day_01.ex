defmodule Aoc2022.Day01 do
  @day "day01"

  def part1() do
    input = Aoc2022.process_input(@day)
    lines = String.split(input, "\n")

    {max_calories, _current_calories} =
      Enum.reduce(
        lines,
        {0, 0},
        fn line, {max, current} ->
          case String.trim(line) do
            "" ->
              if current > max do
                {current, 0}
              else
                {max, 0}
              end

            _ ->
              {max, current + String.to_integer(line)}
          end
        end
      )

    max_calories
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
