defmodule AdventOfCode.Solution.Year2024.Day02 do
  def part1(input) do
    input
    |> parse_reports()
    |> Enum.count(&is_report_safe/1)
  end

  def part2(input) do
    input
    |> parse_reports()
    |> Enum.count(fn report ->
      is_report_safe(report) or
        Enum.any?(0..(length(report) - 1), fn index_to_remove ->
          report
          |> remove_index(index_to_remove)
          |> is_report_safe()
        end)
    end)
  end

  defp parse_reports(input) do
    input
    |> String.trim()
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_report/1)
  end

  defp parse_report(report) do
    report
    |> String.split(~r/\s+/, trim: true)
    |> Enum.map(&String.to_integer/1)
  end

  defp is_report_safe(report) do
    case Enum.chunk_every(report, 2, 1, :discard) do
      [] ->
        false

      windows ->
        if Enum.at(report, 1) == Enum.at(report, 0) do
          false
        else
          check_windows(windows, Enum.at(report, 1) > Enum.at(report, 0))
        end
    end
  end

  defp check_windows(windows, true), do: Enum.all?(windows, &increasing?/1)
  defp check_windows(windows, false), do: Enum.all?(windows, &decreasing?/1)

  defp increasing?([left, right]),
    do: right > left and 1 <= abs(right - left) and abs(right - left) <= 3

  defp decreasing?([left, right]),
    do: right < left and 1 <= abs(right - left) and abs(right - left) <= 3

  defp remove_index(report, index_to_remove) do
    Enum.with_index(report)
    |> Enum.reject(fn {_value, index} -> index == index_to_remove end)
    |> Enum.map(fn {value, _index} -> value end)
  end
end
