package y2025.d04

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "04"

    fun parseGrid(input: List<String>): Triple<Array<Array<Long>>, Int, Int> {
        val height = input.size
        val width = input[0].length
        val grid = Array(height + 2) { Array(width + 2) { 0L } }
        input.forEachIndexed { i, line ->
            line.forEachIndexed { j, c -> grid[i + 1][j + 1] = if (c == '@') 1 else 0 }
        }

        return Triple(grid, height, width)
    }

    fun removeRolls(
        grid: Array<Array<Long>>,
        height: Int,
        width: Int,
    ): Pair<Long, Array<Array<Long>>> {
        val newGrid = grid.map { it.clone() }.toTypedArray()
        var accessible = 0L
        for (i in 1..height) {
            for (j in 1..width) {
                if (
                    grid[i][j] == 1L &&
                        grid[i - 1][j] +
                            grid[i - 1][j - 1] +
                            grid[i][j - 1] +
                            grid[i + 1][j - 1] +
                            grid[i + 1][j] +
                            grid[i + 1][j + 1] +
                            grid[i][j + 1] +
                            grid[i - 1][j + 1] < 4
                ) {
                    accessible++
                    newGrid[i][j] = 0
                }
            }
        }

        return Pair(accessible, newGrid)
    }

    fun part1(input: List<String>): Long {
        val (grid, height, width) = parseGrid(input)

        val (accessible, _) = removeRolls(grid, height, width)
        return accessible
    }

    fun part2(input: List<String>): Long {
        var (grid, height, width) = parseGrid(input)

        var accessible = 0L
        do {
            val (newAccessible, newGrid) = removeRolls(grid, height, width)
            grid = newGrid
            accessible += newAccessible
        } while (newAccessible > 0)

        return accessible
    }

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    val part1Test = part1(testInput)
    checkTest(13, part1Test)
    part1(input).println()

    val part2Test = part2(testInput)
    checkTest(43, part2Test)
    part2(input).println()
}
