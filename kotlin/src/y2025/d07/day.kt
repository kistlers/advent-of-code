package y2025.d07

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "07"

    fun List<String>.parseGrid(): Pair<Int, List<List<Char>>> {
        val grid = this.map(String::toList)
        for (j in grid[0].indices) {
            if (grid[0][j] == 'S') return j to grid
        }
        error("No starting position found")
    }

    fun MutableMap<Int, Long>.addBeam(beam: Int, count: Long) = this.merge(beam, count, Long::plus)

    fun part1And2(input: List<String>): Pair<Long, Long> {
        val (start, grid) = input.parseGrid()
        val height = grid.size
        val width = grid[0].size

        var beams = mapOf(start to 1L)
        var beamSplits = 0L

        for (i in 0 until height - 2) {
            val newBeams = mutableMapOf<Int, Long>()
            for ((beam, count) in beams) {
                when (grid[i + 1][beam]) {
                    '.' -> newBeams.addBeam(beam, count)
                    '^' -> {
                        beamSplits++
                        if (beam > 0) {
                            newBeams.addBeam(beam - 1, count)
                        }
                        if (beam < width - 1) {
                            newBeams.addBeam(beam + 1, count)
                        }
                    }
                    else -> error("Invalid beam position: ${grid[i + 1][beam]}")
                }
            }
            beams = newBeams
        }

        val timelines = beams.values.sum()
        return beamSplits to timelines
    }

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    val (test1, test2) = part1And2(testInput)
    val (part1, part2) = part1And2(input)

    checkTest(21, test1)
    part1.println()

    checkTest(40, test2)
    part2.println()
}
