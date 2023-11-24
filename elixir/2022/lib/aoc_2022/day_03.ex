defmodule Aoc2022.Day03 do
  @day "day03"
  @priorities Enum.to_list(?a..?z) ++ Enum.to_list(?A..?Z)

  def part1() do
    Aoc2022.process_input(@day, :split)
    |> Enum.map(&rucksacks/1)
    |> Enum.sum()
  end

  def rucksacks(line) do
    half = (String.length(line) / 2) |> trunc()
    rucksack_1 = String.slice(line, 0, half) |> string_to_mapset()
    rucksack_2 = String.slice(line, half, half) |> string_to_mapset()
    intersect = MapSet.intersection(rucksack_1, rucksack_2) |> MapSet.to_list() |> hd()
    score(intersect)
  end

  def score(item) do
    Enum.find_index(@priorities, fn c -> c == item end) + 1
  end

  def string_to_mapset(string) when is_binary(string) do
    string
    |> String.to_charlist()
    |> MapSet.new()
  end

  def part2() do
    Aoc2022.process_input(@day, :split)
    |> Enum.chunk_every(3)
    |> Enum.map(&badge/1)
    |> Enum.sum()
  end

  def badge(elves) do
    # IO.inspect(elves)

    intersect =
      Enum.reduce(elves, string_to_mapset(hd(elves)), fn elf, acc ->
        MapSet.intersection(acc, string_to_mapset(elf))
      end)
      |> MapSet.to_list()
      |> hd()

    # IO.inspect(intersect)
    score(intersect)
  end
end

defmodule Mix.Tasks.Day03 do
  use Mix.Task

  def run([]) do
    solution = Aoc2022.Day03.part1()
    IO.puts(solution)
  end

  def run([arg]) do
    case arg do
      "part1" ->
        solution = Aoc2022.Day03.part1()
        IO.puts(solution)

      "part2" ->
        solution = Aoc2022.Day03.part2()
        IO.puts(solution)

      _ ->
        IO.puts("Unknown argument: #{arg}")
        System.halt(1)
    end
  end
end
