defmodule Aoc2022 do
  def process_input(day, :split) do
    input = process_input(day)
    String.split(input, "\n", trim: true)
  end

  def process_input(day) do
    IO.puts("Control-D when finished")
    input = IO.read(:stdio, :all)
    handle_input(day, String.trim(input))
  end

  defp handle_input(day, "") do
    filename = "inputs/" <> day <> "-input.txt"

    case File.read(filename) do
      {:ok, body} ->
        body

      {:error, reason} ->
        IO.puts("Failed to read file #{filename}, #{reason}")
        System.halt(1)
    end
  end

  defp handle_input(_day, input) do
    input
  end
end
