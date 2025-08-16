import io.BufferedReader
import memory.MemoryBuffer
import kotlin.test.Test
import kotlin.test.assertEquals

class BufferedReaderTest {
//    @Test
//    fun `test buffered read with multiple calls`() {
//        val memoryBuffer = MemoryBuffer()
//        val bufferedReader = BufferedReader(memoryBuffer)
//        val data = "This is a test buffer.".encodeToByteArray()
//        val buf = ByteArray(8)
//
//        // Write data to MemoryBuffer
//        memoryBuffer.write(data,, data.size)
//
//        // Read data in chunks
//        val read1 = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(8, read1)
//        assertEquals("This is ", buf.decodeToString())
//
//        val read2 = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(8, read2)
//        assertEquals("a test b", buf.decodeToString())
//
//        val read3 = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(6, read3)
//        assertEquals("uffer.", buf.decodeToString(0, read3))
//
//        // EOF should return 0 bytes read
//        val read4 = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(0, read4)
//    }
//
//    @Test
//    fun `test readLine with mixed line endings`() {
//        val memoryBuffer = MemoryBuffer()
//        val bufferedReader = BufferedReader(memoryBuffer)
//        val data = "Line one\nLine two\rLine three\r\nLine four".encodeToByteArray()
//
//        memoryBuffer.write(data,, data.size)
//
//        val line1 = bufferedReader.readLine()
//        assertEquals("Line one", line1)
//
//        val line2 = bufferedReader.readLine()
//        assertEquals("Line two", line2)
//
//        val line3 = bufferedReader.readLine()
//        assertEquals("Line three", line3)
//
//        val line4 = bufferedReader.readLine()
//        assertEquals("Line four", line4)
//
//        // After all lines, subsequent readLine should return null (EOF)
//        val line5 = bufferedReader.readLine()
//        assertEquals(null, line5)
//    }
//
//    @Test
//    fun `test readToEnd after partial reads`() {
//        val memoryBuffer = MemoryBuffer()
//        val bufferedReader = BufferedReader(memoryBuffer)
//        val data = "Hello World!".encodeToByteArray()
//
//        memoryBuffer.write(data,, data.size)
//
//        // Perform a partial read first
//        val buf = ByteArray(5)
//        val bytesRead = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(5, bytesRead)
//        assertEquals("Hello", buf.decodeToString())
//
//        // Read the remaining data using readToEnd
//        val remainder = mutableListOf<UByte>()
//        val totalRead = bufferedReader.readToEnd(remainder,).getOrThrow()
//        assertEquals(7, totalRead) // " World!" is 7 bytes
//        assertEquals(" World!", remainder.map { it.toByte() }.toByteArray().decodeToString())
//    }
//
//    @Test
//    fun `test read beyond buffer size`() {
//        val memoryBuffer = MemoryBuffer()
//        val bufferedReader = BufferedReader(memoryBuffer)
//        val data = ByteArray(10_000) { (it % 256).toByte() }
//
//        memoryBuffer.write(data,, data.size)
//
//        // Read in chunks larger than BufferedReader's buffer size to test handling
//        val buf = ByteArray(8192)
//        val read1 = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(8192, read1)
//
//        val read2 = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(1808, read2) // 10_000 - 8192 = 1808
//
//        val read3 = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(0, read3) // EOF
//    }
//
//    @Test
//    fun `test handling of zero-length reads`() {
//        val memoryBuffer = MemoryBuffer()
//        val bufferedReader = BufferedReader(memoryBuffer)
//        val data = "Some test data.".encodeToByteArray()
//
//        memoryBuffer.write(data,, data.size)
//
//        // Attempt a zero-length read
//        val zeroBuf = ByteArray(0)
//        val zeroRead = bufferedReader.read(zeroBuf, 0).getOrThrow()
//        assertEquals(0, zeroRead)
//
//        // Check that it doesn't affect subsequent reads
//        val buf = ByteArray(4)
//        val bytesRead = bufferedReader.read(buf, buf.size).getOrThrow()
//        assertEquals(4, bytesRead)
//        assertEquals("Some", buf.decodeToString())
//    }
//
//    @Test
//    fun `test readLine across buffer boundaries`() {
//        val memoryBuffer = MemoryBuffer()
//        val bufferedReader = BufferedReader(memoryBuffer)
//
//        // Write data that exceeds buffer size and has line breaks
//        val largeData = ("Line 1\n" + "X".repeat(8192) + "\nLine 2").encodeToByteArray()
//
//        memoryBuffer.write(largeData,, largeData.size)
//
//        // First line should be correctly read
//        val line1 = bufferedReader.readLine()
//        assertEquals("Line 1", line1)
//
//        // Skipping the large chunk (8000 'X'), line 2 should still be correct
//        val largeBuffer = ByteArray(8192)
//        bufferedReader.read(largeBuffer, 8192) // Reading the large chunk
//        println(largeBuffer.size)
//        println(largeBuffer.decodeToString())
//        // val line2 = bufferedReader.readLine()
//        // assertEquals("Line 2", line2)
//    }
//
//    @Test
//    fun `test reading from empty buffer`() {
//        val memoryBuffer = MemoryBuffer()
//        val bufferedReader = BufferedReader(memoryBuffer)
//
//        // No data has been written
//        val emptyBuf = ByteArray(10)
//        val bytesRead = bufferedReader.read(emptyBuf, emptyBuf.size).getOrThrow()
//        assertEquals(0, bytesRead)
//
//        val line = bufferedReader.readLine()
//        assertEquals(null, line)
//
//        val remainder = mutableListOf<UByte>()
//        val totalRead = bufferedReader.readToEnd(remainder,).getOrThrow()
//        assertEquals(0, totalRead)
//    }
}
