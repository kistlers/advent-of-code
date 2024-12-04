defmodule AdventOfCode.Solution.Year2024.Day03 do
  # Helper function to scan and extract mul pairs
  defp scan_mul(regex, input) do
    Regex.scan(regex, String.trim(input))
  end

  def part1(input) do
    mul_regex = ~r/mul\((\d{1,3}),(\d{1,3})\)/

    scan_mul(mul_regex, input)
    |> Enum.map(fn [_, left, right] ->
      String.to_integer(left) * String.to_integer(right)
    end)
    |> Enum.sum()
  end

  def part2(input) do
    mul_regex = ~r/mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)/

    scan_mul(mul_regex, input)
    |> Enum.reduce({0, true}, fn match, {acc, enabled_do} ->
      case match do
        ["don't()"] ->
          {acc, false}

        ["do()"] ->
          {acc, true}

        [_, left, right] when enabled_do ->
          mul = String.to_integer(left) * String.to_integer(right)
          {acc + mul, enabled_do}

        _ ->
          {acc, enabled_do}
      end
    end)
    |> elem(0)
  end
end
