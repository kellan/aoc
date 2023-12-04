# pivoting my code structure style on day 3

defmodule Day03 do
  @dirs [{1, 0}, {0, 1}, {-1, 0}, {0, -1}, {1, 1}, {-1, 1}, {-1, -1}, {1, -1}]

  def part1() do
    regex =  ~r/[^0-9.]/ # match anything that is not a number or a dot

    # a feel like in a different language I could do this as a single pass
    # but I don't know how to do that in elixir

    lines = lines()
    symbols = lines
    |> Enum.with_index()
    |> Enum.reduce(MapSet.new(), fn {line, lineno}, acc ->
      Regex.scan(regex, line, return: :index)
      |> List.flatten()
      |> Enum.reduce(acc, fn {start, _length}, acc ->
        MapSet.put(acc, {lineno, start})
      end)
    end)

    numbers = lines
    |> Enum.with_index()
    |> Enum.reduce(Map.new(), fn {line, lineno}, acc ->
      Regex.scan(~r/\d+/, line, return: :index)
      |> List.flatten()
      |> Enum.reduce(acc, fn {start, length}, acc ->
        Map.put(acc, {lineno, start}, String.slice(line, start, length))

      end)
    end)

    parts = Enum.map(symbols, fn {lineno, symbol_x} ->
      rows = [lineno - 1, lineno, lineno + 1]
      Enum.filter(Map.keys(numbers), fn {l, s} ->
        l in rows
      end)
      |> Enum.filter(fn {l,s} ->
        num = Map.get(numbers, {l, s})
        cols = for i <- 0..(String.length(num) - 1), do: s + i
        Enum.any?(cols, fn c ->
          c in [symbol_x-1, symbol_x, symbol_x+1]
        end)
      end)
    end)
    |> List.flatten()
    |> Enum.map(fn k ->
      Map.get(numbers, k) |> String.to_integer()
    end)
    |> Enum.sum()

    IO.puts(parts)

  end

  def lines() do
    IO.read(:stdio, :all)
    |> String.split("\n", trim: true)
  end

end
