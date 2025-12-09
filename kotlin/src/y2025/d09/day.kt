package y2025.d09

import checkTest
import org.locationtech.jts.geom.Coordinate
import org.locationtech.jts.geom.GeometryFactory
import println
import readInput
import readTest

fun main() {
    val year = "2025"
    val day = "09"

    val geometryFactory = GeometryFactory()

    fun List<String>.parsePoints(): List<Pair<Double, Double>> =
        this.map { it.split(",").let { (x, y) -> x.toDouble() to y.toDouble() } }

    fun solve(input: List<String>): Pair<Long, Long> {
        val points = input.parsePoints()

        val ring = points.map { Coordinate(it.first, it.second) }.toMutableList()
        ring.add(ring.first())
        val polygon = geometryFactory.createPolygon(ring.toTypedArray())

        val rectangles =
            points.flatMapIndexed { i, p1 ->
                points.drop(i + 1).map { p2 ->
                    val (x1, y1) = p1
                    val (x2, y2) = p2
                    val rectangle =
                        geometryFactory.createPolygon(
                            geometryFactory.createLinearRing(
                                arrayOf(
                                    Coordinate(x1, y1),
                                    Coordinate(x2, y1),
                                    Coordinate(x2, y2),
                                    Coordinate(x1, y2),
                                    Coordinate(x1, y1),
                                )
                            )
                        )

                    // our rectangle is only aligned in the center between the grid points. We need
                    // to add a 0.5 wide boundary to the outside, which is
                    // 2*height*0.5 + 2*width*0.5 + 4*1/4 grid cells == boundary/2+1
                    val area = rectangle.area + rectangle.boundary.length / 2 + 1
                    val isInPolygon = polygon.contains(rectangle)
                    area to isInPolygon
                }
            }

        val part1 = rectangles.maxOf { it.first }
        val part2 = rectangles.filter { it.second }.maxOf { it.first }
        return part1.toLong() to part2.toLong()
    }

    val testInput = readTest(year, day)
    val input = readInput(year, day)

    val (test1, test2) = solve(testInput)
    val (part1, part2) = solve(input)

    checkTest(50, test1)
    part1.println()

    checkTest(24, test2)
    part2.println()
}
