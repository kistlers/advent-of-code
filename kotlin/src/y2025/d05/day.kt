package y2025.d05

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "05"

    fun parseInput(input: List<String>): Pair<List<ClosedRange<Long>>, List<Long>> {
        val rangeLines = input.takeWhile { it.isNotEmpty() }
        val idLines = input.dropWhile { it.isNotEmpty() }.drop(1)

        val ranges =
            rangeLines.map { line ->
                val (from, to) = line.split('-').map(String::toLong)
                from..to
            }
        val ids = idLines.map(String::toLong)
        return ranges to ids
    }

    fun ClosedRange<Long>.countLong(): Long = endInclusive - start + 1

    fun part1(input: List<String>): Long {
        val (ranges, ids) = parseInput(input)

        return ids.count { id -> ranges.any { id in it } }.toLong()
    }

    fun part2(input: List<String>): Long {
        val (ranges, _) = parseInput(input)

        return ranges
            .sortedWith(compareBy({ it.start }, { it.endInclusive }))
            .fold(0L to -1L) { (sum, end), r ->
                when {
                    // current range is fully after the current end -> count all
                    end < r.start -> sum + r.countLong() to r.endInclusive
                    // current range is fully covered -> count nothing
                    end >= r.endInclusive -> sum to end
                    // partial overlap -> count the uncovered tail [end+1..r.end]
                    else -> sum + (r.endInclusive - end) to r.endInclusive
                }
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
