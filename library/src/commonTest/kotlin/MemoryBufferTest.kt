import io.buffered.SlicedByteArray
import kotlin.test.*
import memory.MemoryBuffer
import memory.encodeToSlicedByteArray


class MemoryBufferTest {

    @Test
    fun testWriteAndRead() {
        val buffer = MemoryBuffer()
        val input = "Hello, World!".encodeToSlicedByteArray()

        val writeLen = buffer.write(input, 0, input.size)
        println(writeLen)
        assertEquals(input.size, writeLen)

        val readTarget = SlicedByteArray.allocate(input.size)
        val readLen = buffer.read(readTarget, 0, readTarget.size)
        println(readLen)
        assertEquals(input.size, readLen)
        assertContentEquals(input, readTarget)
    }

    @Test
    fun testReadPartial() {
        val buffer = MemoryBuffer()
        val input = "Data".encodeToSlicedByteArray()
        buffer.write(input, 0, input.size)

        val partial = SlicedByteArray.allocate(2)
        val readLen = buffer.read(partial, 0, partial.size)
        assertEquals(2, readLen)
        assertContentEquals(SlicedByteArray.wrap("Da".encodeToByteArray()), partial)
    }

    @Test
    fun testReadToEnd() {
        val buffer = MemoryBuffer()
        val input = "EndTest".encodeToSlicedByteArray()
        buffer.write(input, 0, input.size)

        val list = mutableListOf<Byte>()
        val result = buffer.readToEnd(list, 0)
        assertEquals(input.size, result)

        val expected = "EndTest".encodeToByteArray().toList()
        assertContentEquals(expected, list)
    }

    @Test
    fun testResetRead() {
        val buffer = MemoryBuffer()
        val input = "abc".encodeToSlicedByteArray()
        buffer.write(input, 0, input.size)

        val first = SlicedByteArray.allocate(2)
        buffer.read(first, 0, 2)
        buffer.resetRead()

        val second = SlicedByteArray.allocate(3)
        buffer.read(second, 0, 3)
        assertContentEquals(input, second)
    }

    @Test
    fun testClear() {
        val buffer = MemoryBuffer()
        val input = "clear".encodeToSlicedByteArray()
        buffer.write(input, 0, input.size)

        buffer.clear()

        val target = SlicedByteArray.allocate(5)
        val result = buffer.read(target, 0, target.size)
        assertEquals(-1, result)
    }

    @Test
    fun testWriteAll() {
        val buffer = MemoryBuffer()
        val input = "WriteAll".encodeToSlicedByteArray()

        buffer.writeAll(input, 0)

        val readTarget = SlicedByteArray.allocate(input.size)
        buffer.resetRead()
        val readLen = buffer.read(readTarget, 0, readTarget.size)
        assertEquals(input.size, readLen)
        assertContentEquals(input, readTarget)
    }

    @Test
    fun testSnapshot() {
        val buffer = MemoryBuffer()
        val input = "Snap".encodeToSlicedByteArray()
        buffer.writeAll(input, 0)

        val snap = buffer.snapshot()
        assertContentEquals("Snap".encodeToByteArray(), snap)
    }

    @Test
    fun testReadBeyondEOF() {
        val buffer = MemoryBuffer()
        val target = SlicedByteArray.allocate(10)
        val result = buffer.read(target, 0, 10)
        assertEquals(-1, result)
    }

    @Test
    fun testWriteNegativeLengthFails() {
        val buffer = MemoryBuffer()
        val data = SlicedByteArray.wrap(byteArrayOf(1, 2, 3))
        assertFailsWith<IllegalArgumentException> { buffer.write(data, 0, -1) }
    }

    @Test
    fun testReadNegativeLengthFails() {
        val buffer = MemoryBuffer()
        val buf = SlicedByteArray.allocate(3)
        assertFailsWith<IllegalArgumentException> { buffer.read(buf, 0, -1) }
    }

    @Test
    fun testWriteLenExceedsDataFails() {
        val buffer = MemoryBuffer()
        val data = SlicedByteArray.wrap(byteArrayOf(1, 2))
        assertFailsWith<IllegalArgumentException> { buffer.write(data, 0, 5) }
    }

    @Test
    fun testReadLenExceedsBufFails() {
        val buffer = MemoryBuffer()
        val buf = SlicedByteArray.allocate(2)
        assertFailsWith<IllegalArgumentException> { buffer.read(buf, 0, 5) }
    }

    @Test
    fun `test write and read`() {
        val memBuffer = MemoryBuffer()
        val data = SlicedByteArray.wrap("Hello, World!".encodeToByteArray())
        val buf1 = SlicedByteArray.allocate(8)

        // Write data
        assertEquals(data.size, memBuffer.write(data, 0, data.size))

        // Read partial data
        val bytesRead1 = memBuffer.read(buf1, 0, buf1.size)
        assertEquals(8, bytesRead1)
        assertContentEquals(SlicedByteArray.wrap("Hello, W".encodeToByteArray()), buf1)

        // Read remaining data
        val buf2 = SlicedByteArray.allocate(8)
        val bytesRead2 = memBuffer.read(buf2, 0, buf2.size)
        assertEquals(5, bytesRead2)
        val head = SlicedByteArray.allocate(bytesRead2)
        buf2.copyInto(head, 0, 0, bytesRead2)
        assertContentEquals(SlicedByteArray.wrap("orld!".encodeToByteArray()), head)
    }

    @Test
    fun `test readToEnd`() {
        val memBuffer = MemoryBuffer()
        val data = SlicedByteArray.wrap("Hello, World!".encodeToByteArray())
        val output = mutableListOf<Byte>()

        // Write data
        assertEquals(data.size, memBuffer.write(data, 0, data.size))

        // Read all remaining data
        val totalRead = memBuffer.readToEnd(output, 0)
        assertEquals(data.size, totalRead)
        assertEquals("Hello, World!".encodeToByteArray().toList(), output)
    }

    @Test
    fun `test clear and resetRead`() {
        val memBuffer = MemoryBuffer()
        val data = SlicedByteArray.wrap("Hello, World!".encodeToByteArray())
        val buf = SlicedByteArray.allocate(8)

        // Write and read some data
        assertEquals(data.size, memBuffer.write(data, 0, data.size))
        memBuffer.read(buf, 0, buf.size)

        // Reset read position and re-read
        memBuffer.resetRead()
        val bytesRead = memBuffer.read(buf, 0, buf.size)
        assertEquals(8, bytesRead)
        assertContentEquals(SlicedByteArray.wrap("Hello, W".encodeToByteArray()), buf)

        // Clear buffer and verify
        memBuffer.clear()
        assertEquals(-1, memBuffer.read(buf, 0, buf.size))
    }

    @Test
    fun `test buffer expansion`() {
        val memBuffer = MemoryBuffer()
        val data = ByteArray(50) { it.toByte() } // 50 bytes of data

        // Write more than initial capacity
        assertEquals(data.size, memBuffer.write(SlicedByteArray.wrap(data), 0, data.size))
        assertEquals(50, memBuffer.snapshot().size)

        // Check buffer content
        assertEquals(data.toList(), memBuffer.snapshot().toList())
    }

    @Test
    fun `test invalid inputs`() {
        val memBuffer = MemoryBuffer()
        val data = ByteArray(10)

        // Negative length
        assertFailsWith<IllegalArgumentException> { memBuffer.write(SlicedByteArray.wrap(data), 0, -1) }
        assertFailsWith<IllegalArgumentException> { memBuffer.read(SlicedByteArray.wrap(data), 0, -1) }

        // Length larger than buffer size
        assertFailsWith<IllegalArgumentException> { memBuffer.write(SlicedByteArray.wrap(data), 0, data.size + 1) }
        assertFailsWith<IllegalArgumentException> { memBuffer.read(SlicedByteArray.wrap(data), 0, data.size + 1) }
    }
}
