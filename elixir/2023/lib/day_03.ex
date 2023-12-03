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
    |> Enum.reduce(MapSet.new(), fn {line, lineno}, acc ->
      Regex.scan(~r/\d+/, line, return: :index)
      |> List.flatten()
      |> Enum.reduce(acc, fn {start, length}, acc ->

        points = Enum.map(start..(start + length - 1), fn n ->
          [lineno, n]
        end)
        |> MapSet.new()

        IO.inspect(points)

        acc = MapSet.union(acc, points)
        #IO.inspect(acc)
      end)
    end)

    IO.inspect(numbers)

    # Enum.each(symbols, fn {lineno, start} ->
    #   Enum.each(@dirs, fn {dx, dy} ->
    #     [d_lineno, d_start] = [lineno + dy, start + dx]

    #   end)
    # end)
    #   Enum.each(@dirs, fn {dx, dy} ->
    #     adj = Enum.at(lines, y + dy) |> String.at(x + dx)
    #     case adj do
    #       "." -> nil
    #       _ -> IO.puts("found #{adj} at #{x + dx}, #{y + dy}")
    #     end
    #   end)
    # end)
    # Enum.each(symbols, fn {x, y} ->
    #   Enum.each(@dirs, fn {dx, dy} ->
    #     adj = Enum.at(lines, y + dy) |> String.at(x + dx)
    #     case adj do
    #       "." -> nil
    #       _ -> IO.puts("found #{adj} at #{x + dx}, #{y + dy}")
    #     end
    #   end)
    # end)

    # lines
    # |> Enum.with_index()
    # |> Enum.each(fn {line, i} ->
    #   #IO.puts("#{i} #{line}")
    #     String.graphemes(line)
    #     |> Enum.with_index()
    #     |> Enum.each(fn {char, j} ->
    #       #IO.puts("#{j},#{i} #{char}")

    #     end)
    # end)


    # # symbols = lines
    # |> Enum.with_index()
    # |> Enum.reduce(MapSet.new(), fn {line, i}, acc ->
    #   IO.puts("#{i} #{line}")
    #   Regex.scan(regex, line, return: :index)
    #   |> List.flatten()
    #   |> Enum.reduce(acc, fn {start, _length}, acc ->
    #     MapSet.put(acc, {start, i})
    #   end)
    # end)

    # IO.inspect(lines)
    # IO.inspect(symbols)

    # Enum.each(symbols, fn {x, y} ->
    #   Enum.each(@dirs, fn {dx, dy} ->
    #     adj = Enum.at(lines, y + dy) |> String.at(x + dx)
    #     case adj do
    #       "." -> nil
    #       _ -> IO.puts("found #{adj} at #{x + dx}, #{y + dy}")
    #     end
    #   end)
    # end)

  end

  def lines() do
    IO.read(:stdio, :all)
    |> String.split("\n", trim: true)
  end

end
