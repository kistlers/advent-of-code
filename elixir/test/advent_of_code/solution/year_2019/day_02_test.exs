defmodule AdventOfCode.Solution.Year2019.Day02Test do
  use ExUnit.Case, async: true

  import AdventOfCode.Solution.Year2019.Day02

  setup do
    [
      test_cases: [
        %{
          input: """
          1,9,10,3,2,3,11,0,99,30,40,50
          """,
          expected_part1: 3500,
          expected_part2: 5000
        },
        # %{
        #   input: """
        #   1,0,0,0,99
        #   """,
        #   expected_part1: 2,
        #   expected_part2: 42
        # },
        # %{
        #   input: """
        #   2,3,0,3,99
        #   """,
        #   expected_part1: 2,
        #   expected_part2: 12
        # },
        # %{
        #   input: """
        #   2,4,4,5,99,0
        #   """,
        #   expected_part1: 2,
        #   expected_part2: 12
        # },
        # %{
        #   input: """
        #   1,1,1,4,99,5,6,0,99
        #   """,
        #   expected_part1: 30,
        #   expected_part2: 12
        # }
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
