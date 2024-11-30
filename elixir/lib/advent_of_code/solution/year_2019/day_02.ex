defmodule AdventOfCode.Solution.Year2019.Day02 do
  def part1(input) do
    memory = parse(input)

    memory = Map.put(memory, 1, 12)
    memory = Map.put(memory, 2, 2)

    execute(memory)
  end

  def part2(input) do
    memory = parse(input)

    results =
      for noun <- 0..99, verb <- 0..99 do
        {execute(Map.merge(memory, %{1 => noun, 2 => verb})), noun * 100 + verb}
      end

    # IO.inspect(results)

    {_, answer} =
      results
      |> Stream.filter(fn {output, _} -> output == 19_690_720 end)
      |> Enum.take(1)
      |> hd

    answer
  end

  def execute(memory, ip \\ 0) do
    # IO.inspect(memory)
    code = Map.fetch!(memory, ip)

    # IO.puts("Got code #{code} at ip #{ip}")

    case code do
      1 ->
        memory = run_op(&+/2, memory, ip)
        execute(memory, ip + 4)

      2 ->
        memory = run_op(&*/2, memory, ip)
        execute(memory, ip + 4)

      99 ->
        # IO.puts("Done, got #{Map.fetch!(memory, 0)}\n\n")
        Map.fetch!(memory, 0)

      _ ->
        raise("Got unexpected code #{code} at ip #{ip}")
    end
  end

  def run_op(op, memory, ip) do
    left = Map.fetch!(memory, ip + 1)
    right = Map.fetch!(memory, ip + 2)
    dest = Map.fetch!(memory, ip + 3)
    result = op.(Map.fetch!(memory, left), Map.fetch!(memory, right))
    Map.put(memory, dest, result)
  end

  def parse(input) do
    input
    |> String.trim()
    |> String.split(",", trim: true)
    |> Stream.map(&String.to_integer/1)
    |> Stream.with_index()
    |> Stream.map(fn {element, index} -> {index, element} end)
    |> Map.new()
  end
end
