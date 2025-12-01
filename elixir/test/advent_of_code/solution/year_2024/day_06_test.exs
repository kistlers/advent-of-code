defmodule AdventOfCode.Solution.Year2024.Day06Test do
  use ExUnit.Case, async: true

  import AdventOfCode.Solution.Year2024.Day06

  setup do
    [
      test_cases: [
        %{
          input: """
          ....#.....
          .........#
          ..........
          ..#.......
          .......#..
          ..........
          .#..^.....
          ........#.
          #.........
          ......#...
          """,
          expected_part1: 41,
          expected_part2: 6
        }
      ]
    ]
  end

#  @tag :skip
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
