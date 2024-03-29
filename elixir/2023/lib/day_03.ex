# pivoting my code structure style on day 3

defmodule Day03 do

  def part1() do
    regex =  ~r/[^0-9.]/ # match anything that is not a number or a dot

    lines = lines()
    [symbols, numbers] = parse(lines,  regex)

    parts = adjacent_numbers(symbols, numbers)

    |> List.flatten()
    |> Enum.map(fn k ->
      Map.get(numbers, k) |> String.to_integer()
    end)
    |> Enum.sum()

    IO.puts(parts)

  end



  def part2() do
    lines = lines()
    [symbols, numbers] = parse(lines,  ~r/\*/)
    parts = adjacent_numbers(symbols, numbers)

    # don't flatten for part 2, to get a adjacency list per symbol
    parts
    |> Enum.filter(fn l ->
      length(l) == 2
    end)
    |> Enum.map(fn [g1, g2] ->
      n1 = Map.get(numbers, g1) |> String.to_integer()
      n2 = Map.get(numbers, g2) |> String.to_integer()
      n1 * n2
    end)
    |> Enum.sum()
    |> IO.puts()
  end

  def adjacent_numbers(symbols, numbers) do
    # for each symbol, check the adjancent rows and columns for numbers
    Enum.map(symbols, fn {lineno, symbol_x} ->
      rows = [lineno - 1, lineno, lineno + 1]
      Enum.filter(Map.keys(numbers), fn {l, _} ->
        l in rows
      end)
      |> Enum.filter(fn {l,s} ->
        num = Map.get(numbers, {l, s})
        # get all the columns a number occupies
        cols = for i <- 0..(String.length(num) - 1), do: s + i
        Enum.any?(cols, fn c ->
          c in [symbol_x-1, symbol_x, symbol_x+1]
        end)
      end)
    end)
  end

  def parse(lines, symbol_regex) do

    # a feel like in a different language I could do this as a single pass
    # but I don't know how to do that in elixir

    # line and column set of each symbol matching the regex
    symbols = lines
    |> Enum.with_index()
    |> Enum.reduce(MapSet.new(), fn {line, lineno}, acc ->
      Regex.scan(symbol_regex, line, return: :index)
      |> List.flatten()
      |> Enum.reduce(acc, fn {start, _length}, acc ->
        MapSet.put(acc, {lineno, start})
      end)
    end)

    # map line and column to the number it contains
    numbers = lines
    |> Enum.with_index()
    |> Enum.reduce(Map.new(), fn {line, lineno}, acc ->
      Regex.scan(~r/\d+/, line, return: :index)
      |> List.flatten()
      |> Enum.reduce(acc, fn {start, length}, acc ->
        Map.put(acc, {lineno, start}, String.slice(line, start, length))

      end)
    end)

    [symbols, numbers]
  end

  def lines() do
    IO.read(:stdio, :all)
    |> String.split("\n", trim: true)
  end

end
