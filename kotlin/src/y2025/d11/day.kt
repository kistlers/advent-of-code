package y2025.d11

import checkTest
import println
import readInput
import readTest
import readTest2

typealias Graph = Map<String, Set<String>>

fun main() {
    val year = "2025"
    val day = "11"

    fun List<String>.parseGraph(): Graph =
        this.map { line -> line.split(": ").let { (from, tos) -> from to tos.split(" ").toSet() } }
            .associate { (from, tos) -> from to tos }

    val memo = mutableMapOf<Triple<String, Boolean, Boolean>, Long>()

    fun Graph.findUniquePathsToOut(
        currentPath: List<Triple<String, Boolean, Boolean>>,
        withDacAndFft: Boolean,
    ): Long {
        val current = currentPath.last()
        val (currentNode, dacVisited, fftVisited) = current

        if (memo.containsKey(current)) {
            return memo.getValue(current)
        }

        val paths =
            this[currentNode]!!
                .filter { Triple(it, dacVisited, fftVisited) !in currentPath }
                .sumOf { next ->
                    if (next == "out") {
                        if (withDacAndFft) {
                            if (dacVisited && fftVisited) {
                                1L
                            } else {
                                0L
                            }
                        } else {
                            1L
                        }
                    } else {
                        val nextPath =
                            currentPath +
                                Triple(
                                    next,
                                    dacVisited || next == "dac",
                                    fftVisited || next == "fft",
                                )
                        findUniquePathsToOut(currentPath = nextPath, withDacAndFft = withDacAndFft)
                    }
                }

        memo[current] = paths

        return paths
    }

    fun part1(input: List<String>): Long {
        memo.clear()

        val graph = input.parseGraph()
        return graph.findUniquePathsToOut(
            currentPath = listOf(Triple("you", false, false)),
            withDacAndFft = false,
        )
    }

    fun part2(input: List<String>): Long {
        memo.clear()

        val graph = input.parseGraph()
        return graph.findUniquePathsToOut(
            currentPath = listOf(Triple("svr", false, false)),
            withDacAndFft = true,
        )
    }

    val testInput = readTest(year, day)
    val test2Input = readTest2(year, day)
    val input = readInput(year, day)

    checkTest(5, part1(testInput))
    part1(input).println()

    checkTest(2, part2(test2Input))
    part2(input).println()
}
