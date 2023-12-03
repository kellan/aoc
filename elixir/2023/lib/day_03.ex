# pivoting my code structure style on day 3

defmodule Day03 do
  @dirs [{1, 0}, {0, 1}, {-1, 0}, {0, -1}, {1, 1}, {-1, 1}, {-1, -1}, {1, -1}]

  def part1() do
    regex =  ~r/[^0-9.]/ # match anything that is not a number or a dot

    lines = lines()
    symbols = lines
    |> Enum.with_index()
    |> Enum.reduce(MapSet.new(), fn {line, i}, acc ->
      IO.puts("#{i} #{line}")
      Regex.scan(regex, line, return: :index)
      |> List.flatten()
      |> Enum.reduce(acc, fn {start, _length}, acc ->
        MapSet.put(acc, {start, i})
      end)
    end)

    # IO.inspect(lines)
    # IO.inspect(symbols)

    Enum.each(symbols, fn {x, y} ->
      Enum.each(@dirs, fn {dx, dy} ->
        adj = Enum.at(lines, y + dy) |> String.at(x + dx)
        case adj do
          "." -> nil
          _ -> IO.puts("found #{adj} at #{x + dx}, #{y + dy}")
        end
      end)
    end)

  end

  def lines() do
    IO.read(:stdio, :all)
    |> String.split("\n", trim: true)
  end

end
