package y2025.d02

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "02"

    fun ids(input: List<String>): Sequence<Long> =
        input.first()
            .split(',')
            .asSequence()
            .flatMap { range ->
                val (start, end) = range.split('-').map(String::toLong)
                (start..end).asSequence()
            }

    fun String.isRepeatingNTimes(n: Int): Boolean =
        length % n == 0 && this == take(length / n).repeat(n)

    fun part1(input: List<String>): Long =
        ids(input)
            .filter { it.toString().isRepeatingNTimes(2) }
            .sum()

    fun part2(input: List<String>): Long =
        ids(input)
            .filter { id ->
                val s = id.toString()
                (2..s.length).any(s::isRepeatingNTimes)
            }
            .sum()

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    checkTest(1227775554, part1(testInput))
    part1(input).println()

    checkTest(4174379265, part2(testInput))
    part2(input).println()
}
