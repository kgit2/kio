import kotlin.test.*
import memory.MemoryBuffer

class MemoryBufferTest {

    @Test
    fun testWriteAndRead() {
        val buffer = MemoryBuffer()
        val input = "Hello, World!".encodeToByteArray()
        val readTarget = ByteArray(input.size)

        val writeResult = buffer.write(input,, input.size)
        assertTrue(writeResult.isSuccess)
        assertEquals(input.size, writeResult.getOrThrow())

        val readResult = buffer.read(readTarget, readTarget.size)
        assertTrue(readResult.isSuccess)
        assertEquals(input.size, readResult.getOrThrow())
        assertContentEquals(input, readTarget)
    }

    @Test
    fun testReadPartial() {
        val buffer = MemoryBuffer()
        val input = "Data".encodeToByteArray()
        buffer.write(input,, input.size)

        val partial = ByteArray(2)
        val readResult = buffer.read(partial, partial.size)
        assertTrue(readResult.isSuccess)
        assertEquals(2, readResult.getOrThrow())
        assertContentEquals(byteArrayOf('D'.code.toByte(), 'a'.code.toByte()), partial)
    }

    @Test
    fun testReadToEnd() {
        val buffer = MemoryBuffer()
        val input = "EndTest".encodeToByteArray()
        buffer.write(input,, input.size)

        val list = mutableListOf<UByte>()
        val result = buffer.readToEnd(list,)
        assertTrue(result.isSuccess)
        assertEquals(input.size, result.getOrThrow())
        assertContentEquals(input.map { it.toUByte() }, list)
    }

    @Test
    fun testResetRead() {
        val buffer = MemoryBuffer()
        val input = "abc".encodeToByteArray()
        buffer.write(input,, input.size)

        val first = ByteArray(2)
        buffer.read(first, 2)
        buffer.resetRead()

        val second = ByteArray(3)
        buffer.read(second, 3)
        assertContentEquals(input, second)
    }

    @Test
    fun testClear() {
        val buffer = MemoryBuffer()
        val input = "clear".encodeToByteArray()
        buffer.write(input,, input.size)

        buffer.clear()

        val target = ByteArray(5)
        val result = buffer.read(target, target.size)
        assertTrue(result.isSuccess)
        assertEquals(0, result.getOrThrow())
    }

    @Test
    fun testWriteAll() {
        val buffer = MemoryBuffer()
        val input = "WriteAll".encodeToByteArray()

        val result = buffer.writeAll(input,)
        assertTrue(result.isSuccess)

        val readTarget = ByteArray(input.size)
        buffer.resetRead()
        buffer.read(readTarget, readTarget.size)

        assertContentEquals(input, readTarget)
    }

    @Test
    fun testSnapshot() {
        val buffer = MemoryBuffer()
        val input = "Snap".encodeToByteArray()
        buffer.writeAll(input,)

        val snap = buffer.snapshot()
        assertContentEquals(input, snap)
    }

    @Test
    fun testReadBeyondEOF() {
        val buffer = MemoryBuffer()
        val target = ByteArray(10)
        val result = buffer.read(target, 10)
        assertTrue(result.isSuccess)
        assertEquals(0, result.getOrThrow())
    }

    @Test
    fun testWriteNegativeLengthFails() {
        val buffer = MemoryBuffer()
        val data = byteArrayOf(1, 2, 3)
        val result = buffer.write(data,, -1)
        assertTrue(result.isFailure)
    }

    @Test
    fun testReadNegativeLengthFails() {
        val buffer = MemoryBuffer()
        val buf = ByteArray(3)
        val result = buffer.read(buf, -1)
        assertTrue(result.isFailure)
    }

    @Test
    fun testWriteLenExceedsDataFails() {
        val buffer = MemoryBuffer()
        val data = byteArrayOf(1, 2)
        val result = buffer.write(data,, 5)
        assertTrue(result.isFailure)
    }

    @Test
    fun testReadLenExceedsBufFails() {
        val buffer = MemoryBuffer()
        val buf = ByteArray(2)
        val result = buffer.read(buf, 5)
        assertTrue(result.isFailure)
    }

    @Test
    fun `test write and read`() {
        val memBuffer = MemoryBuffer()
        val data = "Hello, World!".encodeToByteArray()
        val buf = ByteArray(8)

        // Write data
        assertEquals(data.size, memBuffer.write(data,, data.size).getOrNull())

        // Read partial data
        val bytesRead1 = memBuffer.read(buf, buf.size).getOrNull()
        assertEquals(8, bytesRead1)
        assertEquals("Hello, W", buf.decodeToString())

        // Read remaining data
        val bytesRead2 = memBuffer.read(buf, buf.size).getOrNull()
        assertEquals(5, bytesRead2)
        assertEquals("orld!, W", buf.decodeToString())
    }

    @Test
    fun `test readToEnd`() {
        val memBuffer = MemoryBuffer()
        val data = "Hello, World!".encodeToByteArray()
        val output = mutableListOf<UByte>()

        // Write data
        assertEquals(data.size, memBuffer.write(data,, data.size).getOrNull())

        // Read all remaining data
        val totalRead = memBuffer.readToEnd(output,).getOrNull()
        assertEquals(data.size, totalRead)
        assertEquals(data.toList(), output.map { it.toByte() })
    }

    @Test
    fun `test clear and resetRead`() {
        val memBuffer = MemoryBuffer()
        val data = "Hello, World!".encodeToByteArray()
        val buf = ByteArray(8)

        // Write and read some data
        assertEquals(data.size, memBuffer.write(data,, data.size).getOrNull())
        memBuffer.read(buf, buf.size)

        // Reset read position and re-read
        memBuffer.resetRead()
        val bytesRead = memBuffer.read(buf, buf.size).getOrNull()
        assertEquals(8, bytesRead)
        assertEquals("Hello, W", buf.decodeToString())

        // Clear buffer and verify
        memBuffer.clear()
        assertEquals(0, memBuffer.read(buf, buf.size).getOrNull())
    }

    @Test
    fun `test buffer expansion`() {
        val memBuffer = MemoryBuffer()
        val data = ByteArray(50) { it.toByte() } // 50 bytes of data

        // Write more than initial capacity
        assertEquals(data.size, memBuffer.write(data,, data.size).getOrNull())
        assertEquals(50, memBuffer.snapshot().size)

        // Check buffer content
        assertEquals(data.toList(), memBuffer.snapshot().toList())
    }

    @Test
    fun `test invalid inputs`() {
        val memBuffer = MemoryBuffer()
        val data = ByteArray(10)

        // Negative length
        assertFailsWith<IllegalArgumentException> { memBuffer.write(data,, -1).getOrThrow() }
        assertFailsWith<IllegalArgumentException> { memBuffer.read(data, -1).getOrThrow() }

        // Length larger than buffer size
        assertFailsWith<IllegalArgumentException> { memBuffer.write(data,, data.size + 1).getOrThrow() }
        assertFailsWith<IllegalArgumentException> { memBuffer.read(data, data.size + 1).getOrThrow() }
    }
}
