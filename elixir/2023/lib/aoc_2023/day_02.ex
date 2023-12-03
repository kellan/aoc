defmodule Aoc2023.Day02 do
  @day "day02"
  @rules %{red: 12, green: 13, blue: 14}

  def part1() do
    Aoc2023.process_input(@day, :split)
    |> Enum.map(&parse_game/1)
    |> Enum.filter(fn {game_id, game} ->
      !Enum.any?(game, fn {color, num} ->
        num > Map.get(@rules, color)
      end)
    end)
    |> Enum.map(&elem(&1, 0))
    |> Enum.sum()
  end

  def parse_game(line) do
    [game_str, draws_str] = String.split(line, ":")
    [_, game_id] = Regex.run(~r/Game (\d+)/, game_str)

    matches = Regex.scan(~r/(\d+) (\w+)/, draws_str)
    game = Enum.reduce(matches, %{}, fn [_, num, color], acc ->
      num = String.to_integer(num)
      color = String.to_atom(color)
      if num > Map.get(acc, color, 0) do
        Map.put(acc, color, num)
      else
        acc
      end
    end)

    #IO.inspect([String.to_integer(game_id), game])
    {String.to_integer(game_id), game}
  end

  def part2() do
    Aoc2023.process_input(@day, :split)
    |> Enum.map(&parse_game/1)
    |> Enum.map(fn {game_id, game} ->
      Map.get(game, :red, 0) * Map.get(game, :blue, 0) * Map.get(game, :green, 0)
      end)
    |> Enum.sum()
  end
end

defmodule Mix.Tasks.Day02 do
  use Mix.Task

  def run([]) do
    solution = Aoc2023.Day02.part1()
    IO.puts(solution)
  end

  def run([arg]) do
    case arg do
      "part1" ->
        solution = Aoc2023.Day02.part1()
        IO.puts(solution)

      "part2" ->
        solution = Aoc2023.Day02.part2()
        IO.puts(solution)

      _ ->
        IO.puts("Unknown argument: #{arg}")
        System.halt(1)
    end
  end
end
