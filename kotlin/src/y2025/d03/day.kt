package y2025.d03

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "03"

    fun findJoltage(input: List<String>, numberOfBanks: Int): Long =
        input.sumOf {
            val digits = it.toCharArray().toList().map { d -> d.digitToInt() }
            var joltage = ""
            var index = 0
            for (i in numberOfBanks downTo 1) {
                val maxIndex = (index..digits.size - i).maxBy { i -> digits[i] }
                joltage += digits[maxIndex]
                index = maxIndex + 1
            }
            joltage.toLong()
        }

    fun part1(input: List<String>): Long = findJoltage(input, 2)

    fun part2(input: List<String>): Long = findJoltage(input, 12)

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    val part1Test = part1(testInput)
    checkTest(357, part1Test)
    part1(input).println()

    val part2Test = part2(testInput)
    checkTest(3121910778619, part2Test)
    part2(input).println()
}
