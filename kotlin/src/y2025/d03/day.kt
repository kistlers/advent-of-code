package y2025.d03

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "03"

    fun findJoltage(input: List<String>, numberOfBanks: Int): Long =
        input.sumOf { line ->
            val digits = line.map(Char::digitToInt)
            var from = 0
            var value = 0L
            for (remaining in numberOfBanks downTo 1) {
                val maxIndex = (from..digits.size - remaining).maxByOrNull { idx -> digits[idx] }!!
                value = value * 10 + digits[maxIndex]
                from = maxIndex + 1
            }
            value
        }

    fun part1(input: List<String>): Long = findJoltage(input, 2)

    fun part2(input: List<String>): Long = findJoltage(input, 12)

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    checkTest(357, part1(testInput))
    part1(input).println()

    checkTest(3121910778619, part2(testInput))
    part2(input).println()
}
