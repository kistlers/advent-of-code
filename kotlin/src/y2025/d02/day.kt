package y2025.d02

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "02"

    fun allIds(input: List<String>): List<Long> =
        input[0].split(",").flatMap {
            val split = it.split("-")
            (split[0].toLong()..split[1].toLong()).toList()
        }

    fun isRepeatingNTimes(s: String, n: Int): Boolean =
        s.length % n == 0 && s == s.take(s.length / n).repeat(n)

    fun part1(input: List<String>): Long =
        allIds(input)
            .filter {
                val s = it.toString()
                // if the length is even, and first half == second half
                isRepeatingNTimes(s, 2)
            }
            .sum()

    fun part2(input: List<String>): Long =
        allIds(input)
            .filter {
                val s = it.toString()
                (2..s.length).any { n -> isRepeatingNTimes(s, n) }
            }
            .sum()

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    val part1Test = part1(testInput)
    checkTest(1227775554, part1Test)
    part1(input).println()

    val part2Test = part2(testInput)
    checkTest(4174379265, part2Test)
    part2(input).println()
}
