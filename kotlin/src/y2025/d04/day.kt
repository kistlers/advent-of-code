package y2025.d04

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "04"

    fun parseGrid(input: List<String>): Triple<Array<BooleanArray>, Int, Int> {
        val height = input.size
        val width = input.first().length
        val grid = Array(height) { i -> BooleanArray(width) { j -> input[i][j] == '@' } }
        return Triple(grid, height, width)
    }

    fun removeRolls(
        grid: Array<BooleanArray>,
        height: Int,
        width: Int,
    ): Pair<Long, Array<BooleanArray>> {
        fun neighborCount(r: Int, c: Int): Int {
            var count = 0
            for (dr in -1..1) {
                for (dc in -1..1) {
                    if (dr == 0 && dc == 0) continue
                    val nr = r + dr
                    val nc = c + dc
                    if (nr in 0 until height && nc in 0 until width && grid[nr][nc]) count++
                }
            }
            return count
        }

        val newGrid = Array(height) { r -> grid[r].clone() }
        var accessible = 0L
        for (i in 0 until height) {
            for (j in 0 until width) {
                if (grid[i][j] && neighborCount(i, j) < 4) {
                    accessible++
                    newGrid[i][j] = false
                }
            }
        }

        return accessible to newGrid
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

    checkTest(13, part1(testInput))
    part1(input).println()

    checkTest(43, part2(testInput))
    part2(input).println()
}
