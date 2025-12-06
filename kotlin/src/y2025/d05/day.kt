package y2025.d05

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "05"

    fun parseInput(input: List<String>): Pair<List<ClosedRange<Long>>, List<Long>> {
        val emptyLineIndex = input.indexOf("")
        val ranges =
            input.take(emptyLineIndex).map {
                val from = it.substringBefore("-").toLong()
                val to = it.substringAfter("-").toLong()
                from..to
            }
        val ids = input.drop(emptyLineIndex + 1).map { it.toLong() }
        return ranges to ids
    }

    fun ClosedRange<Long>.countLong(): Long = endInclusive - start + 1

    fun part1(input: List<String>): Long {
        val (ranges, ids) = parseInput(input)

        return ids.count { id -> ranges.any { range -> id in range } }.toLong()
    }

    fun part2(input: List<String>): Long {
        val (ranges, _) = parseInput(input)

        return ranges
            .sortedWith(compareBy({ it.start }, { it.endInclusive }))
            .fold(0L to -1L) { (idCount, end), range ->
                // the current range is fully after the current end -> count all
                if (end < range.start) {
                    return@fold idCount + range.countLong() to range.endInclusive
                }

                // the current range is fully covered -> count no elements
                if (end >= range.endInclusive) {
                    return@fold idCount to range.endInclusive
                }

                // partially covered, count elements in range [end+1, range.endInclusive]
                return@fold idCount + (end + 1..range.endInclusive).countLong() to
                    range.endInclusive
            }
            .first
    }

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    checkTest(3, part1(testInput))
    part1(input).println()

    checkTest(14, part2(testInput))
    part2(input).println()
}
