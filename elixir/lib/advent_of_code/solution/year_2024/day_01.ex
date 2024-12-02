defmodule AdventOfCode.Solution.Year2024.Day01 do
  def part1(input) do
    parse_to_sorted_columns(input)
    |> calculate_column_differences()
    |> Enum.sum()
  end

  def part2(input) do
    [left, right] = parse_to_sorted_columns(input)

    left
    |> Enum.map(&calculate_weight(&1, right))
    |> Enum.sum()
  end

  defp parse_to_sorted_columns(input) do
    input
    |> String.trim()
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_line_to_integers/1)
    |> transpose()
    |> Enum.map(&Enum.sort/1)
  end

  defp parse_line_to_integers(line) do
    line
    |> String.split(~r/\s+/, trim: true)
    |> Enum.map(&String.to_integer/1)
  end

  defp transpose(rows) do
    rows
    |> Enum.zip()
    |> Enum.map(&Tuple.to_list/1)
  end

  defp calculate_column_differences(columns) do
    columns
    |> Enum.zip()
    |> Enum.map(fn {a, b} -> abs(b - a) end)
  end

  defp calculate_weight(left_item, right) do
    left_item * Enum.count(right, fn right_item -> right_item == left_item end)
  end
end
