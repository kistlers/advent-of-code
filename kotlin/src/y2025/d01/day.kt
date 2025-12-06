package y2025.d01

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "01"

    fun findZeroes(rotations: List<Long>): Long =
        rotations
            .runningFold(50L) { acc, curr -> (acc + curr).mod(100L) }
            .drop(1) // exclude the initial starting position
            .count { it == 0L }
            .toLong()

    /** parse to plus or minus rotations, then fold to find all zeroes */
    fun part1(input: List<String>): Long =
        findZeroes(
            input.map { line ->
                when (val dir = line.first()) {
                    'R' -> line.drop(1).toLong()
                    'L' -> -line.drop(1).toLong()
                    else -> error("Invalid input: $line (unknown direction '$dir')")
                }
            }
        )

    /**
     * parse to a list of n+/- rotations, the fold to find all zeroes with the same logic as part1
     */
    fun part2(input: List<String>): Long =
        findZeroes(
            input.flatMap { line ->
                val dir = line.first()
                val count = line.drop(1).toLong()
                val step = when (dir) {
                    'R' -> 1L
                    'L' -> -1L
                    else -> error("Invalid input: $line (unknown direction '$dir')")
                }
                // expand into unit rotations to count intermediate zeroes
                List(count.toInt()) { step }
            }
        )

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    checkTest(3, part1(testInput))
    part1(input).println()

    checkTest(6, part2(testInput))
    part2(input).println()
}
