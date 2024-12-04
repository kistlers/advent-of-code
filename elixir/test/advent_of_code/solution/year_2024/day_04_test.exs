defmodule AdventOfCode.Solution.Year2024.Day04Test do
  use ExUnit.Case, async: true

  import AdventOfCode.Solution.Year2024.Day04

  setup do
    [
      test_cases: [
        %{
          input: """
          MMMSXXMASM
          MSAMXMSMSA
          AMXSXMAAMM
          MSAMASMSMX
          XMASAMXAMM
          XXAMMXXAMA
          SMSMSASXSS
          SAXAMASAAA
          MAMMMXMMMM
          MXMXAXMASX
          """,
          expected_part1: 18,
          expected_part2: 9
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

#  @tag :skip
  test "part2", %{test_cases: test_cases} do
    Enum.each(test_cases, fn %{input: input, expected_part2: expected} ->
      result = part2(input)
      assert result == expected
    end)
  end
end
