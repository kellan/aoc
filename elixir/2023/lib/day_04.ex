defmodule Day04 do

  def part1() do
    lines = lines()

    # Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    cards = cards(lines)
    cards
    |> Enum.map(fn inter ->
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

    cards = cards(lines)
    |> Enum.with_index()
    |> Enum.map(fn {winning, i} ->
      winning = MapSet.size(winning)
      {i, winning}
    end)
    |> Map.new()

    IO.inspect(cards)
    counts = cards
    |> Enum.reduce(%{}, fn {i, winning}, acc ->
      acc = Map.update(acc, i, 1, fn x -> x + 1 end)

      acc = if winning > 0 do
        #Map.update(acc, i+1, 1, fn x -> x + 1 end)
        Enum.reduce(i+1..i+winning-1, acc, fn i, acc ->
          Map.update(acc, i, 1, fn x -> x + 1 end)
        end)
      else
        acc
      end

    end)

    IO.inspect(counts)

  end

  @spec cards(any()) :: list()
  def cards(lines) do
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
      MapSet.intersection(winning, draws)
    end)
  end

  def lines() do
    IO.read(:stdio, :all)
    |> String.split("\n", trim: true)
  end

end
