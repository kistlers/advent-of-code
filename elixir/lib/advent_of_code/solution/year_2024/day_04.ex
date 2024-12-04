defmodule AdventOfCode.Solution.Year2024.Day04 do
  def part1(input) do
    {grid, height, width} = parse_grid(input)

    generate_coords(0, height - 1, 0, width - 1)
    |> Enum.map(fn {row, col} ->
      cond do
        char_at(grid, row, col) == "X" ->
          find_xmas_part1_all_dirs(grid, height, width, row, col)

        true ->
          0
      end
    end)
    |> Enum.sum()
  end

  def part2(input) do
    {grid, height, width} = parse_grid(input)

    generate_coords(1, height - 2, 1, width - 2)
    |> Enum.count(fn {row, col} ->
      cond do
        char_at(grid, row, col) == "A" ->
          find_x_mas_part2(grid, height, width, row, col)

        true ->
          false
      end
    end)
  end

  defp generate_coords(row_min, row_max, col_min, col_max) do
    row_min..row_max
    |> Enum.flat_map(fn row ->
      col_min..col_max
      |> Enum.map(fn col ->
        {row, col}
      end)
    end)
  end

  defp find_x_mas_part2(grid, height, width, row, col) do
    ul = char_at(grid, row - 1, col - 1)
    ur = char_at(grid, row - 1, col + 1)
    ll = char_at(grid, row + 1, col - 1)
    lr = char_at(grid, row + 1, col + 1)

    case {ul, ur, ll, lr} do
      {"M", "M", "S", "S"} ->
        # M.M
        # .A.
        # S.S
        true

      {"M", "S", "M", "S"} ->
        # M.S
        # .A.
        # M.S
        true

      {"S", "S", "M", "M"} ->
        # S.S
        # .A.
        # M.M
        true

      {"S", "M", "S", "M"} ->
        # S.M
        # .A.
        # S.M
        true

      _ ->
        false
    end
  end

  defp find_xmas_part1_all_dirs(grid, height, width, row, col) do
    [{0, 1}, {-1, 1}, {-1, 0}, {-1, -1}, {0, -1}, {1, -1}, {1, 0}, {1, 1}]
    |> Enum.count(fn dir ->
      find_xmas_part1(grid, height, width, {row, col}, dir, ["X", "M", "A", "S"])
    end)
  end

  # always treat as found if no characters remaining
  defp find_xmas_part1(grid, height, width, {row, col}, {d_row, d_col}, []) do
    true
  end

  defp find_xmas_part1(grid, height, width, {row, col}, {d_row, d_col}, [next | remaining]) do
    cond do
      row == -1 ->
        false

      row == height ->
        false

      col == -1 ->
        false

      col == width ->
        false

      char_at(grid, row, col) == next ->
        find_xmas_part1(
          grid,
          height,
          width,
          {row + d_row, col + d_col},
          {d_row, d_col},
          remaining
        )

      true ->
        false
    end
  end

  defp char_at(grid, row, col) do
    Enum.at(Enum.at(grid, row), col)
  end

  defp parse_grid(input) do
    lines =
      input
      |> String.trim()
      |> String.split("\n", trim: true)

    height = length(lines)
    width = String.length(Enum.at(lines, 0))

    grid =
      lines
      |> Enum.map(fn line ->
        line
        |> String.graphemes()
      end)

    {grid, height, width}
  end
end
