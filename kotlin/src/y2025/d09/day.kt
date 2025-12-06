package y2025.d09

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "09"

    fun part1(input: List<String>): Long = input.size.toLong()

    fun part2(input: List<String>): Long = input.size.toLong()

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    checkTest(3, part1(testInput))
    part1(input).println()

    checkTest(6, part2(testInput))
    part2(input).println()
}
