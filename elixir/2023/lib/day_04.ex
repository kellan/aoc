defmodule Day04 do

  def part1() do
    lines = lines()

    # Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    lines
    |> Enum.map(fn line ->
        #IO.inspect(line)
        [_, cards] = String.split(line, ":")
        [winning, draws] = String.split(cards, "|")
        |> Enum.map(fn s ->
          Regex.scan(~r/(\d+)/, s)
          |> Enum.map(fn [_, num] -> num end)
          |> MapSet.new()
        end)
        inter = MapSet.intersection(winning, draws)
        score = if MapSet.size(inter) > 0 do
          2 ** (MapSet.size(inter) - 1)
        else
          0
        end
        score
      end)
    |> Enum.sum()
    |> IO.inspect()
  end



  def part2() do
    lines = lines()
  end


  def lines() do
    IO.read(:stdio, :all)
    |> String.split("\n", trim: true)
  end

end
