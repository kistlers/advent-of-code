defmodule AdventOfCode.Solution.Year2024.Day01Test do
  use ExUnit.Case, async: true

  import AdventOfCode.Solution.Year2024.Day01

  setup do
    [
      test_cases: [
        %{
          input: """
          3   4
          4   3
          2   5
          1   3
          3   9
          3   3
          """,
          expected_part1: 11,
          expected_part2: 31
        }
      ]
    ]
  end

  # @tag :skip
  test "part1", %{test_cases: test_cases} do
    Enum.each(test_cases, fn %{input: input, expected_part1: expected} ->
      result = part1(input)
      assert result == expected
    end)
  end

  # @tag :skip
  test "part2", %{test_cases: test_cases} do
    Enum.each(test_cases, fn %{input: input, expected_part2: expected} ->
      result = part2(input)
      assert result == expected
    end)
  end
end
