defmodule AdventOfCode.Solution.Year2024.Day03Test do
  use ExUnit.Case, async: true

  import AdventOfCode.Solution.Year2024.Day03

  setup do
    [
      test_cases: [
        %{
          input: """
          xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
          """,
          expected_part1: 161,
          expected_part2: 48
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
