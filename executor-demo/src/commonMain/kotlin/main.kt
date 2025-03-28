import io.BufferedReader
import memory.MemoryBuffer

//fun main() {
//    val buf = ByteArray(1024)
//    Stdin.read(buf, 1024).map {
//        println("Read ${it} bytes")
//        Stdout.write(buf.sliceArray(0 until it.toInt()), it)
//            .map {
//                println("Wrote $it bytes")
//            }
//    }
//}

fun main() {
    val memoryBuffer = MemoryBuffer()
    val bufferedReader = BufferedReader(memoryBuffer)
    val data = "This is a test buffer.".encodeToByteArray()
    val buf = ByteArray(8)

    // Write data to MemoryBuffer
    memoryBuffer.write(data,, data.size)

    // Read data in chunks
    val read1 = bufferedReader.read(buf, buf.size).getOrThrow()
    println("Read $read1 bytes: ${buf.decodeToString()}")
    // assertEquals(8, read1)
    // assertEquals("This is ", buf.decodeToString())

    val read2 = bufferedReader.read(buf, buf.size).getOrThrow()
    println("Read $read2 bytes: ${buf.decodeToString()}")
    // assertEquals(8, read2)
    // assertEquals("a test b", buf.decodeToString())

    val read3 = bufferedReader.read(buf, buf.size).getOrThrow()
    println("Read $read3 bytes: ${buf.decodeToString(0, read3)}")
    // assertEquals(6, read3)
    // assertEquals("uffer.", buf.decodeToString(0, read3))

    // EOF should return 0 bytes read
    val read4 = bufferedReader.read(buf, buf.size).getOrThrow()
    println("Read $read4 bytes: ${buf.decodeToString(0, 8)}")
    // assertEquals(0, read4)
}
