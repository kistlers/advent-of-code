import java.math.BigInteger
import java.security.MessageDigest
import kotlin.io.path.Path
import kotlin.io.path.readText

/** Reads lines from the given input txt file. */
fun readInput(year: String, day: String) =
    Path("src/y$year/d$day/input.txt").readText().trim().lines()

/** Reads lines from the given test txt file. */
fun readTest(year: String, day: String) =
    Path("src/y$year/d$day/test.txt").readText().trim().lines()

fun checkTest(expected: Int, actual: Int) =
    check(expected == actual) { "Expected: $expected, Actual: $actual" }

/** Converts string to md5 hash. */
fun String.md5() =
    BigInteger(1, MessageDigest.getInstance("MD5").digest(toByteArray()))
        .toString(16)
        .padStart(32, '0')

/** The cleaner shorthand for printing output. */
fun Any?.println() = println(this)
