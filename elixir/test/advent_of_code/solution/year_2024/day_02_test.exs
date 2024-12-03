defmodule AdventOfCode.Solution.Year2024.Day02Test do
  use ExUnit.Case, async: true

  import AdventOfCode.Solution.Year2024.Day02


  setup do
    [
      test_cases: [
        %{
          input: """
          7 6 4 2 1
          1 2 7 8 9
          9 7 6 2 1
          1 3 2 4 5
          8 6 4 4 1
          1 3 6 7 9
          """,
          expected_part1: 2,
          expected_part2: 4
        }
      ]
    ]
  end

  @tag :skip
  test "part1", %{test_cases: test_cases} do
    Enum.each(test_cases, fn %{input: input, expected_part1: expected} ->
      result = part1(input)
      assert result == expected
    end)
  end

  @tag :skip
  test "part2", %{test_cases: test_cases} do
    Enum.each(test_cases, fn %{input: input, expected_part2: expected} ->
      result = part2(input)
      assert result == expected
    end)
  end
end
