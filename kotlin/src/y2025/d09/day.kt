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

    data class Point(val x: Double, val y: Double)

    fun List<String>.parsePoints(): List<Point> =
        map { it.split(",").let { (x, y) -> Point(x.toDouble(), y.toDouble()) } }

    fun List<Point>.toPolygon(factory: GeometryFactory) =
        factory.createPolygon((this.map { Coordinate(it.x, it.y) } + Coordinate(first().x, first().y)).toTypedArray())

    fun GeometryFactory.rectangle(p1: Point, p2: Point) =
        createPolygon(
            createLinearRing(
                arrayOf(
                    Coordinate(p1.x, p1.y),
                    Coordinate(p2.x, p1.y),
                    Coordinate(p2.x, p2.y),
                    Coordinate(p1.x, p2.y),
                    Coordinate(p1.x, p1.y)
                )
            )
        )

    fun solve(input: List<String>): Pair<Long, Long> {
        val points = input.parsePoints()

        val polygon = points.toPolygon(geometryFactory)

        var maxAny = Double.NEGATIVE_INFINITY
        var maxInside = Double.NEGATIVE_INFINITY

        points.forEachIndexed { i, p1 ->
            for (p2 in points.drop(i + 1)) {
                val rectangle = geometryFactory.rectangle(p1, p2)

                // our rectangle is only aligned in the center between the grid points. We need
                // to add a 0.5 wide boundary to the outside, which is
                // 2*height*0.5 + 2*width*0.5 + 4*1/4 grid cells == boundary/2+1
                val area = rectangle.area + rectangle.boundary.length / 2 + 1
                if (area > maxAny) maxAny = area
                if (polygon.contains(rectangle) && area > maxInside) maxInside = area
            }
        }

        return maxAny.toLong() to maxInside.toLong()
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
