package y2025.d10

import checkTest
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "10"

    fun Int.setBit(pos: Int): Int = this or (1 shl pos)

    fun List<String>.part1ParseInput(): Sequence<Pair<Int, List<Int>>> {
        return this.asSequence().mapIndexed { index, line ->
            val index1 = line.indexOf(']')
            val index2 = line.indexOf('{')

            val indicatorString = line.substring(1, index1)
            var indicators = 0
            for ((pos, ch) in indicatorString.toList().withIndex()) {
                if (ch == '#') indicators = indicators.setBit(pos)
            }

            val wiringsString = line.substring(index1 + 2, index2 - 1)
            val wirings =
                wiringsString
                    .split(' ')
                    .map {
                        val wiringString = it.drop(1).dropLast(1)
                        var wiring = 0
                        for (pos in wiringString.split(',').map(String::toInt)) {
                            wiring = wiring.setBit(pos)
                        }
                        wiring
                    }
                    .toList()

            indicators to wirings
        }
    }

    fun Int.part1Neighbours(wirings: List<Int>, nextDistance: Long): List<Pair<Int, Long>> =
        wirings.map { (this xor it) to nextDistance }

    fun part1Bfs(targetIndicators: Int, wirings: List<Int>): Long {
        val queue = mutableListOf(0 to 0L)
        val visited = mutableSetOf<Int>()

        while (true) {
            val (current, distance) = queue.removeFirst()
            visited.add(current)
            if (current == targetIndicators) {
                return distance
            }

            val neighbours =
                current
                    .part1Neighbours(wirings, distance + 1)
                    .filter { !visited.contains(it.first) }
                    .filter { !queue.contains(it) }
            queue.addAll(neighbours)
        }
    }

    fun part1(input: List<String>): Long =
        input
            .part1ParseInput()
            .mapIndexed { index, (indicators, wirings) ->
                val distance = part1Bfs(indicators, wirings)
                println("Part1 line $index: $distance")
                distance
            }
            .sum()

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    checkTest(7, part1(testInput))
    part1(input).println()

    checkTest(33, 33L)
    15883L.println()
}
