defmodule AdventOfCode.Solution.Year2024.Day06 do
  def part1(input) do
    grid =
      input
      |> String.trim()
      |> String.split("\n", trim: true)
      |> Enum.map(&String.graphemes/1)

    guard = find_guard(grid)

    move_guard(grid, guard, {-1, 0})
  end

  defp find_guard(grid) do
    Enum.find_value(grid, fn row ->
      case Enum.find_index(row, fn cell -> cell == "^" end) do
        nil -> nil
        col -> {Enum.find_index(grid, fn r -> r == row end), col}
      end
    end)
  end

  # Start moving the guard and mark the path
  def move_guard(grid, start_pos, direction) do
    # Start by marking the initial position as visited
    grid = mark_visited(grid, start_pos)

    # Move recursively
    {final_grid, _final_position} = move_guard_recursively(grid, start_pos, direction)

    # Count the '*' marks in the final grid
    count_visited_positions(final_grid)
  end

  # Recursive movement of the guard
  defp move_guard_recursively(grid, {x, y}, {dx, dy}) do
    # Check if the guard has moved off the grid
    if out_of_bounds?(grid, x + dx, y + dy) do
      # Return the grid and the last position
      {mark_visited(grid, {x, y}), {x, y}}
    else
      # Mark the current position as visited
      grid = mark_visited(grid, {x, y})

      case Enum.at(grid, x + dx) |> Enum.at(y + dy) do
        "#" ->
          # Hit an obstacle, turn 90 degrees to the right
          new_direction = turn_right({dx, dy})
          # Move to the next position in the new direction
          move_guard_recursively(
            grid,
            {x + elem(new_direction, 0), y + elem(new_direction, 1)},
            new_direction
          )

        _ ->
          # No obstacle, move in the current direction
          move_guard_recursively(grid, {x + elem({dx, dy}, 0), y + elem({dx, dy}, 1)}, {dx, dy})
      end
    end
  end

  # Check if the guard has moved out of the grid's bounds
  defp out_of_bounds?(grid, x, y) do
    x < 0 or x >= length(grid) or y < 0 or y >= length(Enum.at(grid, 0))
  end

  # Turn the guard 90 degrees to the right
  defp turn_right({dx, dy}) do
    case {dx, dy} do
      # Up -> Right
      {-1, 0} -> {0, 1}
      # Right -> Down
      {0, 1} -> {1, 0}
      # Down -> Left
      {1, 0} -> {0, -1}
      # Left -> Up
      {0, -1} -> {-1, 0}
    end
  end

  # Mark the current position as visited in the grid
  defp mark_visited(grid, {x, y}) do
    List.update_at(grid, x, fn row ->
      List.update_at(row, y, fn _ -> "*" end)
    end)
  end

  # Count how many '*' characters are in the grid
  defp count_visited_positions(grid) do
    grid
    # Flatten the 2D grid into a list
    |> Enum.flat_map(& &1)
    # Count the '*' characters
    |> Enum.count(&(&1 == "*"))
  end

  def part2(_input) do
  end
end
