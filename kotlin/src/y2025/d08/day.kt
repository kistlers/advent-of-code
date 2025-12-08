package y2025.d08

import checkTest
import println
import readInput
import readTest
import kotlin.math.pow
import kotlin.math.sqrt

typealias Vertex = Triple<Long, Long, Long>

typealias Edge = Pair<Int, Int>

fun main() {
    val year = "2025"
    val day = "08"

    fun Vertex.distance(other: Vertex): Double {
        val (x1, y1, z1) = this
        val (x2, y2, z2) = other

        return sqrt(
            (x2.toDouble() - x1).pow(2) + (y2.toDouble() - y1).pow(2) + (z2.toDouble() - z1).pow(2)
        )
    }

    fun List<String>.parseInput(): Pair<List<Vertex>, List<Edge>> {
        val vertices =
            this.map {
                it.split(",").let { (a, b, c) -> Vertex(a.toLong(), b.toLong(), c.toLong()) }
            }
        val edges =
            vertices
                .flatMapIndexed { i1, v1 ->
                    vertices.drop(i1 + 1).mapIndexed { i2, v2 ->
                        Edge(i1, i1 + 1 + i2) to v1.distance(v2)
                    }
                }
                .sortedBy { it.second }
                .map { it.first }
        return vertices to edges
    }

    fun connectBoxes(input: List<String>, part1NumEdgesToConnect: Int): Pair<Long, Long> {
        val (vertices, edges) = input.parseInput()

        val connectedComponents = vertices.indices.associateWith { mutableSetOf(it) }.toMutableMap()
        val representative = vertices.indices.associateWith { it }.toMutableMap()

        var part1: Long = 0

        edges.forEachIndexed { edgeIndex, (i1, i2) ->
            val r1 = representative[i1]!!
            val r2 = representative[i2]!!
            if (r1 < r2) {
                for (addedVertex in connectedComponents[r2]!!) {
                    representative[addedVertex] = r1
                    connectedComponents[r1]!!.add(addedVertex)
                }
                connectedComponents.remove(r2)
            } else if (r1 > r2) {
                for (addedVertex in connectedComponents[r1]!!) {
                    representative[addedVertex] = r2
                    connectedComponents[r2]!!.add(addedVertex)
                }
                connectedComponents.remove(r1)
            }

            if (connectedComponents.size == 1) {
                // when the last two components are connected, calculate part2
                val x1 = vertices[i1].first
                val x2 = vertices[i2].first
                return part1 to (x1 * x2)
            }

            if (edgeIndex == part1NumEdgesToConnect - 1) {
                // when at edge number part1NumEdgesToConnect, calculate part1
                part1 =
                    connectedComponents.values
                        .toSet()
                        .sortedByDescending { it.size }
                        .take(3)
                        .map { it.size.toLong() }
                        .reduce { acc, size -> acc * size }
            }
        }

        error("Should have returned in the forEach")
    }

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    val (test1, test2) = connectBoxes(testInput, 10)
    val (part1, part2) = connectBoxes(input, 1000)

    checkTest(40, test1)
    part1.println()

    checkTest(25272, test2)
    part2.println()
}
