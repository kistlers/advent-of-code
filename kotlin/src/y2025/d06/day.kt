package y2025.d06

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "06"

    val whitespace = "\\s+".toRegex()

    fun parseOps(opsLine: String): List<String> = opsLine.trim().split(whitespace)

    fun grandTotal(problems: List<List<Long>>, ops: List<String>): Long =
        problems.zip(ops).sumOf { (problem, op) ->
            when (op) {
                "+" -> problem.sum()
                "*" -> problem.fold(1L) { acc, v -> acc * v }
                else -> error("Invalid operation: $op")
            }
        }

    fun part1(input: List<String>): Long {
        val rows: List<List<Long>> =
            input.dropLast(1).map { line -> line.trim().split(whitespace).map(String::toLong) }

        val width = rows.first().size
        val problems: List<List<Long>> = List(width) { c -> rows.map { it[c] } }

        val ops = parseOps(input.last())
        return grandTotal(problems, ops)
    }

    fun part2(input: List<String>): Long {
        val opsLine = input.last()
        val maxLen = input.maxOf { it.length }

        val starts = buildList {
            opsLine.forEachIndexed { idx, ch -> if (ch == '+' || ch == '*') add(idx) }
            // sentinel to delimit the last problem window exactly like original logic
            add(maxLen + 1)
        }

        val padded = input.map { it.padEnd(maxLen, ' ') }

        val elementsPerProblem = starts.windowed(2) { (a, b) -> b - a - 1 }

        val rows = padded.dropLast(1)

        val problems: List<List<Long>> =
            elementsPerProblem.mapIndexed { i, width ->
                val start = starts[i]
                // Build each column number by reading downwards across rows
                List(width) { j ->
                    val col = start + j
                    rows.fold(0L) { acc, line ->
                        val ch = line[col]
                        if (ch == ' ') acc else acc * 10 + ch.digitToInt().toLong()
                    }
                }
            }

        val ops = parseOps(opsLine)
        return grandTotal(problems, ops)
    }

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    checkTest(4277556, part1(testInput))
    part1(input).println()

    checkTest(3263827, part2(testInput))
    part2(input).println()
}
