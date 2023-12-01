defmodule Aoc2023.Day01 do
  @day "day01"

  def part1() do
    Aoc2023.process_input(@day, :split)
    |> Enum.map(&first_last_digit/1)
    |> Enum.sum()
  end

  def first_last_digit(line) do
    matches = Regex.scan(~r/(\d)/, line)
    "#{tl(Enum.at(matches, 0))}#{tl(Enum.at(matches, -1))}"
    |> String.to_integer()
  end

  def part2() do
    Aoc2023.process_input(@day, :split)
    |> Enum.map(&first_last_words_digits/1)
    |> Enum.map(&String.to_integer/1)
    |> Enum.sum()
  end

  def first_last_words_digits(line) do
    regex = ~r/(one|two|three|four|five|six|seven|eight|nine|zero|\d)/
    matches = Regex.scan(regex, line)
    first = word_to_digit(hd(tl(hd(matches))))
    #IO.inspect(first)

    {match_status, match} = SuffixMatcher.find_matching_suffix(line, regex)
    last = word_to_digit(hd(tl(match)))
    #IO.inspect(last)

    "#{first}#{last}"

  end

  def word_to_digit(word) do
    case word do
      "one" -> 1
      "two" -> 2
      "three" -> 3
      "four" -> 4
      "five" -> 5
      "six" -> 6
      "seven" -> 7
      "eight" -> 8
      "nine" -> 9
      "zero" -> 0
      _ -> String.to_integer(word)
    end
  end
end


#
# this seems like a horrible way to do this
#
defmodule SuffixMatcher do
  def find_matching_suffix(input, regex) do
    find_matching_suffix(input, regex, 1)
  end

  defp find_matching_suffix(_, _, 0) do
    {:no_match, ""}
  end

  defp find_matching_suffix(input, regex, length) do
    suffix = String.slice(input, -length..-1)

    if matched = Regex.run(regex, suffix) do
      matched = hd(matched)
      {:match, [suffix, matched]}
    else
      find_matching_suffix(input, regex, length+1)
    end
  end
end



defmodule Mix.Tasks.Day01 do
  use Mix.Task

  def run([]) do
    solution = Aoc2023.Day01.part1()
    IO.puts(solution)
  end

  def run([arg]) do
    case arg do
      "part1" ->
        solution = Aoc2023.Day01.part1()
        #IO.puts(solution)

      "part2" ->
        solution = Aoc2023.Day01.part2()
        IO.puts(solution)

      _ ->
        IO.puts("Unknown argument: #{arg}")
        System.halt(1)
    end
  end
end
