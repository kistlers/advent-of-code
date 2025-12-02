package y2025.d05

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "05"

    fun part1(input: List<String>): Long = input.size.toLong()

    fun part2(input: List<String>): Long = input.size.toLong()

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    val part1Test = part1(testInput)
    checkTest(3, part1Test)
    part1(input).println()

    val part2Test = part2(testInput)
    checkTest(6, part2Test)
    part2(input).println()
}
