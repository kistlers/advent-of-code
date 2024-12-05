defmodule AdventOfCode.Solution.Year2024.Day05Test do
  use ExUnit.Case, async: true

  import AdventOfCode.Solution.Year2024.Day05

  setup do
    [
      test_cases: [
        %{
          input: """
          47|53
          97|13
          97|61
          97|47
          75|29
          61|13
          75|53
          29|13
          97|29
          53|29
          61|53
          97|53
          61|29
          47|13
          75|47
          97|75
          47|61
          75|61
          47|29
          75|13
          53|13

          75,47,61,53,29
          97,61,53,29,13
          75,29,13
          75,97,47,61,53
          61,13,29
          97,13,75,29,47
          """,
          expected_part1: 143,
          expected_part2: 123
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
