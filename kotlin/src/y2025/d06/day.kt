package y2025.d06

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "06"

    fun parseOps(opsLine: String): List<String> = opsLine.split("\\s+".toRegex())

    fun grandTotal(problems: List<List<Long>>, ops: List<String>): Long =
        problems
            .zip(ops)
            .map { (problem, op) ->
                return@map when (op) {
                    "+" -> problem.sum()
                    "*" -> problem.reduce { acc, l -> acc * l }
                    else -> error("Invalid operation: $op")
                }
            }
            .sum()

    fun part1(input: List<String>): Long {
        val numProblems = input.first().trim().split("\\s+".toRegex())
        val problems = List(numProblems.size) { mutableListOf<Long>() }
        for (line in input.take(input.size - 1)) {
            for ((i, value) in line.trim().split("\\s+".toRegex()).withIndex()) {
                problems[i].add(value.toLong())
            }
        }

        val ops = parseOps(input.last())
        return grandTotal(problems, ops)
    }

    fun part2(input: List<String>): Long {
        val startColumns = mutableListOf<Int>()
        for ((col, symbol) in input.last().withIndex()) {
            when (symbol) {
                '*',
                '+' -> startColumns.add(col)
            }
        }
        val maxLineLength = input.maxOf { it.length }
        // mark the end of the last column by the start of the non-existent next problem
        startColumns.add(maxLineLength + 1)

        val paddedInput = input.map { it.padEnd(maxLineLength, ' ') }

        val elementsPerProblem = startColumns.windowed(2).map { it[1] - it[0] - 1 }

        val problems = elementsPerProblem.map { Array(it) { 0L }.toMutableList() }
        for ((i, problem) in problems.withIndex()) {
            val startColumn = startColumns[i]
            val endColumnExclusive = startColumns[i + 1] - 1
            for (line in paddedInput.take(paddedInput.size - 1)) {
                val substring = line.substring(startColumn, endColumnExclusive).toList()
                for ((j, digitString) in substring.withIndex()) {
                    if (digitString == ' ') {
                        continue
                    }

                    val digit = digitString.toString().toLong()
                    if (problem[j] == 0L) {
                        problem[j] = digit.toString().toLong()
                    } else {
                        problem[j] = problem[j] * 10 + digit
                    }
                }
            }
        }

        val ops = parseOps(input.last())
        return grandTotal(problems, ops)
    }

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    checkTest(4277556, part1(testInput))
    part1(input).println()

    checkTest(3263827, part2(testInput))
    part2(input).println()
}
