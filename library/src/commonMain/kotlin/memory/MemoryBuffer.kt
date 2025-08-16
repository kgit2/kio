package memory

import io.Read
import io.Write
import io.buffered.SlicedByteArray

class MemoryBuffer : Read, Write {
    // 内部存储使用 ByteArray 实现高效访问
    private var buffer = SlicedByteArray.allocate(INITIAL_SIZE)
    private var readPos = 0
    private var writePos = 0
    private var size = 0  // 当前有效数据长度

    override fun read(buf: SlicedByteArray, offset: Int, len: Int): Int {
        if (len == 0) return 0
        require(len >= 0 && offset >= 0 && offset + len <= buf.size) {
            "Invalid offset/length: offset=$offset len=$len buf.size=${buf.size}"
        }

        val available = size - readPos
        return when {
            available <= 0 -> -1 // EOF
            else -> {
                val toCopy = minOf(len, available)
                val endPos = readPos + toCopy
                buffer.copyInto(buf, offset, readPos, endPos)
                readPos = endPos
                toCopy
            }
        }
    }

    override fun readToEnd(buf: MutableList<UByte>, offset: Int): Int {
        require(offset >= 0 && offset <= buf.size) { "Invalid offset: $offset, buf.size=${buf.size}" }
        val available = size - readPos
        if (available <= 0) return -1

        buf.addAll(offset, List(available) { buffer[readPos + it].toUByte() })
        readPos = size
        return available
    }

    override fun write(buf: SlicedByteArray, offset: Int, len: Int): Int {
        require(len >= 0 && offset >= 0 && offset + len <= buf.size) {
            "Invalid offset/length: offset=$offset len=$len buf.size=${buf.size}"
        }

        ensureCapacityFor(len)
        buf.copyInto(buffer, writePos, offset, offset + len)
        writePos += len
        size = maxOf(size, writePos)
        return len
    }

    override fun writeAll(buf: SlicedByteArray, offset: Int) {
        require(offset >= 0 && offset <= buf.size) { "Invalid offset: $offset, buf.size=${buf.size}" }
        val toWrite = buf.size - offset
        if (toWrite == 0) return

        ensureCapacityFor(toWrite)
        buf.copyInto(buffer, writePos, offset, offset + toWrite)
        writePos += toWrite
        size = maxOf(size, writePos)
    }

    override fun flush() = Unit

    // 内存管理方法
    fun resetRead() {
        readPos = 0
    }

    fun clear() {
        buffer = SlicedByteArray.allocate(buffer.size.coerceAtLeast(INITIAL_SIZE)) // 清空时保留扩容后的大小
        readPos = 0
        writePos = 0
        size = 0
    }

    private fun compact() {
        if (readPos == 0) return
        val remaining = size - readPos
        if (remaining > 0) {
            buffer.copyInto(buffer, 0, readPos, size) // safe overlap copy
        }
        readPos = 0
        writePos = remaining
        size = remaining
    }

    private fun ensureCapacityFor(len: Int) {
        require(len >= 0) { "len must be >= 0: $len" }

        // fast path: enough tail space
        if (writePos + len <= buffer.size) return

        val remaining = size - readPos
        // If compaction would make enough room, do a single in-place move
        if (remaining + len <= buffer.size) {
            compact()
            return
        }

        // Otherwise grow once and copy the unread bytes into the new buffer
        val newSize = maxOf(buffer.size shl 1, remaining + len)
        val newBuf = SlicedByteArray.allocate(newSize)
        if (remaining > 0) {
            buffer.copyInto(newBuf, 0, readPos, size)
        }
        buffer = newBuf
        readPos = 0
        writePos = remaining
        size = remaining
    }

    /**
     * Return a copy of the current effective data [0, size).
     * Independent of readPos; useful for assertions/debugging.
     */
    fun snapshot(): ByteArray {
        val out = ByteArray(size)
        var i = 0
        while (i < size) {
            out[i] = buffer[i].toByte()
            i++
        }
        return out
    }

    companion object {
        const val INITIAL_SIZE = 16
    }
}

fun String.encodeToSlicedByteArray(): SlicedByteArray =
    SlicedByteArray.wrap(this.encodeToByteArray())
