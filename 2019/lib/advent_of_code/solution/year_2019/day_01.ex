defmodule AdventOfCode.Solution.Year2019.Day01 do
  def part1(input) do
    calculate_fuel_sum(input, &calculate_fuel/1)
  end

  def part2(input) do
    calculate_fuel_sum(input, &calculate_fuel_recursive/1)
  end

  def calculate_fuel(mass) do
    trunc(mass / 3) - 2
  end

  def calculate_fuel_recursive(mass) do
    fuel = max(0, trunc(mass / 3) - 2)
    if fuel == 0 do
      fuel
    else
      fuel + calculate_fuel_recursive(fuel)
    end
  end

  def calculate_fuel_sum(input, calculate_fuel_func) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&String.to_integer/1)
    |> Enum.map(calculate_fuel_func)
    |> Enum.sum()
  end
end
