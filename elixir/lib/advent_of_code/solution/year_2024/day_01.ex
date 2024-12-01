defmodule AdventOfCode.Solution.Year2024.Day01 do
  def part1(input) do
    columns = parse_to_sorted_columns(input)

    columns
    |> Stream.zip()
    |> Enum.map(fn {a, b} -> abs(b - a) end)
    |> Enum.sum()
  end

  def part2(input) do
    columns = parse_to_sorted_columns(input)

    left = Enum.at(columns, 0)
    right = Enum.at(columns, 1)

    left
    |> Stream.map(fn left_item ->
      left_item * Enum.count(right, fn right_item -> right_item == left_item end)
    end)
    |> Enum.sum()
  end

  def parse_to_sorted_columns(input) do
    input
    |> String.trim()
    |> String.split("\n", trim: true)
    |> Stream.map(fn
      line ->
        String.split(line, ~r/\s+/, trim: true)
        |> Enum.map(&String.to_integer/1)
    end)
    |> Enum.to_list()
    # transpose rows to columns
    |> Stream.zip()
    |> Stream.map(&Tuple.to_list/1)
    |> Stream.map(&Enum.sort/1)
    |> Enum.to_list()
  end
end
