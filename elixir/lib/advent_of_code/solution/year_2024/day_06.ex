defmodule AdventOfCode.Solution.Year2024.Day06 do
  def part1(input) do
    grid = parse_grid_part1(input)

    guard = find_guard_part1(grid)

    move_guard(grid, guard, {-1, 0})
  end

  defp parse_grid_part1(input) do
    input
    |> String.trim()
    |> String.split("\n", trim: true)
    |> Enum.map(&String.graphemes/1)
  end

  defp find_guard_part1(grid) do
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
          move_guard_recursively(grid, {x + dx, y + dy}, {dx, dy})
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

  def part2(input) do
    grid = parse_grid_part2(input)

    guard = find_guard_part2(grid)

    # Iterate over every tile in the grid
    grid
    |> Enum.with_index()
    |> Enum.flat_map(fn {row, row_idx} ->
      Enum.with_index(row)
      |> Enum.filter(fn {{tile, _}, _col_idx} -> tile == "." end)
      |> Enum.map(fn {{_tile, _}, col_idx} ->
        # Copy the grid and set the current tile to "#"
        modified_grid = set_tile_blocked(grid, row_idx, col_idx)

        # Simulate the guard's movement on the modified grid
        # Start moving "up"
        case find_guard_loop_part2(modified_grid, guard, {-1, 0}) do
          1 ->
            IO.puts("Created a loop with tile at (#{row_idx},#{col_idx})")
            1
          0 -> 0
        end
      end)
    end)
    |> Enum.sum()
  end

  # Modify a specific tile in the grid
  defp set_tile_blocked(grid, row_idx, col_idx) do
    List.update_at(grid, row_idx, fn row ->
      List.update_at(row, col_idx, fn {_tile, directions} ->
        {"#", directions}
      end)
    end)
  end

  # Simulate the guard's movement
  defp find_guard_loop_part2(grid, {row, col}, dir) do
    simulate_part2(grid, {row, col}, dir)
  end

  defp simulate_part2(grid, {row, col}, {dr, dc}) do
    if out_of_bounds_part2?(grid, {row + dr, col + dc}) do
      # Guard moved off the map
      0
    else
      {_tile, visited_dirs} = get_tile(grid, {row, col})

      if Map.get(visited_dirs, {dr, dc}) do
        # Guard stuck in a loop
        1
      else
        # Mark this direction as visited
        grid = update_visited(grid, {row, col}, {dr, dc})

        new_direction =
          case Enum.at(grid, row + dr) |> Enum.at(col + dc) do
            {"#", _} ->
              # Hit an obstacle, turn 90 degrees to the right
              turn_right({dr, dc})

            _ ->
              # No obstacle, move in the current direction
              {dr, dc}
          end

        new_dr = elem(new_direction, 0)
        new_dc = elem(new_direction, 1)

        # Continue simulating
        simulate_part2(grid, {row + new_dr, col + new_dc}, new_direction)
      end
    end
  end

  # Mark a direction as visited
  defp update_visited(grid, {row, col}, dir) do
    List.update_at(grid, row, fn row ->
      List.update_at(row, col, fn {tile, visited} ->
        {tile, Map.put(visited, dir, true)}
      end)
    end)
  end

  # Get the tile at the given position
  defp get_tile(grid, {row, col}) do
    Enum.at(Enum.at(grid, row), col)
  end

  # Check if the guard is out of bounds
  defp out_of_bounds_part2?(grid, {row, col}) do
    row < 0 or col < 0 or row >= length(grid) or col >= length(Enum.at(grid, 0))
  end

  def parse_grid_part2(input) do
    directions = %{
      # Up
      {-1, 0} => false,
      # Down
      {1, 0} => false,
      # Right
      {0, 1} => false,
      # Left
      {0, -1} => false
    }

    input
    |> String.trim()
    |> String.split("\n", trim: true)
    |> Enum.map(fn row ->
      row
      |> String.graphemes()
      |> Enum.map(fn tile ->
        {tile, Map.new(directions)}
      end)
    end)
  end

  defp find_guard_part2(grid) do
    Enum.find_value(grid, fn row ->
      case Enum.find_index(row, fn {cell, _} -> cell == "^" end) do
        nil -> nil
        col -> {Enum.find_index(grid, fn r -> r == row end), col}
      end
    end)
  end
end
