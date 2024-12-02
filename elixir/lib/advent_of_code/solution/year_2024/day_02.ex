defmodule AdventOfCode.Solution.Year2024.Day02 do
  def part1(input) do
    parse_reports(input)
    |> Enum.filter(fn report ->
      is_report_safe(report)
    end)
    |> Enum.count()
  end

  def part2(input) do
    parse_reports(input)
    |> Enum.filter(fn report ->
      if is_report_safe(report) do
        true
      else
        0..length(report)
        |> Enum.any?(fn index_to_remove ->
          report_without_index =
            report
            |> Enum.with_index()
            |> Enum.reject(fn {_value, index} -> index == index_to_remove end)
            |> Enum.map(fn {value, _index} -> value end)
            is_report_safe(report_without_index)
        end)
      end
    end)
    |> Enum.count()
  end

  def parse_reports(input) do
    input
    |> String.trim()
    |> String.split("\n", trim: true)
    |> Enum.map(fn report ->
      report |> String.split(~r/\s+/, trim: true) |> Enum.map(&String.to_integer/1)
    end)
  end

  def is_report_safe(report) do
    if Enum.at(report, 1) == Enum.at(report, 0) do
      false
    else
      # IO.inspect(report, charlists: :as_lists)
      windows = Enum.chunk_every(report, 2, 1, :discard)
      # IO.inspect(windows, charlists: :as_lists)

      if Enum.at(report, 1) > Enum.at(report, 0) do
        # increasing
        windows
        |> Enum.all?(fn window ->
          left = Enum.at(window, 0)
          right = Enum.at(window, 1)
          right > left && 1 <= abs(right - left) && abs(right - left) <= 3
        end)
      else
        # decreasing
        windows
        |> Enum.all?(fn window ->
          left = Enum.at(window, 0)
          right = Enum.at(window, 1)
          right < left && 1 <= abs(right - left) && abs(right - left) <= 3
        end)
      end
    end
  end
end
