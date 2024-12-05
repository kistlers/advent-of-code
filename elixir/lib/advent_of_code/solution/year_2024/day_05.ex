defmodule AdventOfCode.Solution.Year2024.Day05 do
  def part1(input) do
    {rules, updates} = parse_input(input)

    find_valid_or_invalid(rules, updates, true)
    |> Enum.map(fn update -> Enum.at(update, div(length(update), 2)) end)
    |> Enum.sum()
  end

  def part2(input) do
    {rules, updates} = parse_input(input)

    find_valid_or_invalid(rules, updates, false)
    |> Enum.map(fn update ->
      graph = build_graph_subset(rules, update)

      Enum.sort(update, &edge_exists?(graph, &1, &2))
    end)
    |> Enum.map(fn update -> Enum.at(update, div(length(update), 2)) end)
    |> Enum.sum()
  end

  defp parse_input(input) do
    [split_rules, split_updates] =
      input
      |> String.trim()
      |> String.split("\n\n", trim: true)

    rules =
      split_rules
      |> String.split("\n", trim: true)
      |> Enum.map(fn rule ->
        rule
        |> String.split("|", trim: true)
        |> Enum.map(&String.to_integer/1)
        |> List.to_tuple()
      end)

    updates =
      split_updates
      |> String.split("\n", trim: true)
      |> Enum.map(fn update ->
        update
        |> String.split(",", trim: true)
        |> Enum.map(&String.to_integer/1)
      end)

    {rules, updates}
  end

  defp find_valid_or_invalid(rules, updates, find_valid) do
    updates
    |> Enum.filter(fn update ->
      graph = build_graph_subset(rules, update)
      update_is_valid(graph, update) == find_valid
    end)
  end

  defp build_graph_subset(rules, update) do
    rules
    |> Enum.filter(fn {from, to} -> Enum.member?(update, from) && Enum.member?(update, to) end)
    |> to_adjacency_list()
  end

  defp update_is_valid(graph, update) do
    Enum.chunk_every(update, 2, 1, :discard)
    |> Enum.all?(fn [from, to] ->
      edge_exists?(graph, from, to) and not edge_exists?(graph, to, from)
    end)
  end

  defp to_adjacency_list(edges) do
    Enum.reduce(edges, %{}, fn {from, to}, acc ->
      Map.update(acc, from, MapSet.new([to]), &MapSet.put(&1, to))
    end)
  end

  defp edge_exists?(graph, from, to) do
    bfs(graph, [from], MapSet.new(), to)
  end

  defp bfs(_graph, [], _visited, _to), do: false

  defp bfs(graph, [current | queue], visited, to) do
    cond do
      current == to -> true
      true ->
        neighbors = Map.get(graph, current, MapSet.new())
        unvisited_neighbors = MapSet.difference(neighbors, visited)

        # Use the list of unvisited neighbors as the new queue, avoiding `++` for efficiency
        bfs(graph, MapSet.to_list(unvisited_neighbors) ++ queue, MapSet.put(visited, current), to)
    end
  end

end
