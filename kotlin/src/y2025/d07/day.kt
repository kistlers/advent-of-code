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
        val start = grid.first().indexOf('S')
        if (start >= 0) {
            return start to grid
        }
        error("No starting position found")
    }

    fun MutableMap<Int, Long>.addBeam(beam: Int, count: Long) = this.merge(beam, count, Long::plus)

    fun part1And2(input: List<String>): Pair<Long, Long> {
        val (start, grid) = input.parseGrid()
        val height = grid.size
        val width = grid.first().size

        var beams = mapOf(start to 1L)
        var beamSplits = 0L

        for (i in 1 until height - 1) {
            val row = grid[i]
            val newBeams = mutableMapOf<Int, Long>()
            for ((beam, count) in beams) {
                when (val cell = row[beam]) {
                    '.' -> newBeams.addBeam(beam, count)
                    '^' -> {
                        beamSplits++
                        if (beam > 0) {
                            newBeams.addBeam(beam - 1, count)
                        }
                        if (beam + 1 < width) {
                            newBeams.addBeam(beam + 1, count)
                        }
                    }
                    else -> error("Invalid beam position: $cell")
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
