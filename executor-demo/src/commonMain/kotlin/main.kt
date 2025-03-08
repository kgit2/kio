import io.Stdin
import io.Stdout

fun main() {
    val buf = ByteArray(1024)
    Stdin.read(buf, 1024u).map {
        println("Read ${it} bytes")
        Stdout.write(buf.sliceArray(0 until it.toInt()), it)
            .map {
                println("Wrote $it bytes")
            }
        Stdout.flush().getOrThrow()
    }
}
