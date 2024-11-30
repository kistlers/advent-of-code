defmodule AdventOfCode.Solution.Year2019.Day01Test do
  use ExUnit.Case, async: true

  import AdventOfCode.Solution.Year2019.Day01

  setup do
    [
      input: """
      12
      14
      1969
      100756
      """
    ]
  end

  # @tag :skip
  test "part1", %{input: input} do
    result = part1(input)

    assert result == 2 + 2 + 654 + 33583
  end

  # @tag :skip
  test "part2", %{input: input} do
    result = part2(input)

    assert result == 2 + 2 + 966 + 50346
  end
end
