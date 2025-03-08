import io.Stdin
import io.Stdout
import kotlin.test.Test

class Test {
    @Test
    fun test() {
        Stdout.write("Hello, world!\n".encodeToByteArray(),)
            .map {
                println("Wrote $it bytes")
            }
        Stdout.flush()
    }

    @Test
    fun readWrite() {
        val buf = ByteArray(1024)
        Stdin.read(buf,).map {
            println("Read ${it} bytes")
            Stdout.write(buf.sliceArray(0 until it.toInt()),)
                .map {
                    println("Wrote $it bytes")
                }
            Stdout.flush()
        }
    }
}
