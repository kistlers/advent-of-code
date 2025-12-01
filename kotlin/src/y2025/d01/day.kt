package y2025.d01

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "01"

    fun findZeroes(rotations: List<Int>) =
        rotations
            .fold(50 to 0) { acc, curr ->
                val next = (acc.first + curr).mod(100)
                if (next == 0) 0 to (acc.second + 1) else next to acc.second
            }
            .second

    /** parse to plus or minus rotations, then fold to find all zeroes */
    fun part1(input: List<String>): Int =
        findZeroes(
            input.map {
                when (it[0]) {
                    'R' -> it.substring(1).toInt()
                    'L' -> -it.substring(1).toInt()
                    else -> error("Invalid input: $it")
                }
            }
        )

    /**
     * parse to a list of n+/- rotations, the fold to find all zeroes with the same logic as part1
     */
    fun part2(input: List<String>): Int =
        findZeroes(
            input.flatMap {
                when (it[0]) {
                    'R' -> (1..it.substring(1).toInt()).map { 1 }
                    'L' -> (1..it.substring(1).toInt()).map { -1 }
                    else -> error("Invalid input: $it")
                }
            }
        )

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    val part1Test = part1(testInput)
    checkTest(3, part1Test)
    part1(input).println()

    val part2Test = part2(testInput)
    checkTest(6, part2Test)
    part2(input).println()
}
