package memory

import io.Read
import io.Write
import kotlin.math.max

class MemoryBuffer : Read, Write {
    // 内部存储使用 ByteArray 实现高效访问
    private var buffer = ByteArray(INITIAL_SIZE)
    private var readPos = 0
    private var writePos = 0
    private var size = 0  // 当前有效数据长度

    override fun read(buf: ByteArray, offset: Int, len: Int): Result<Int> {
        if (len < 0 || buf.size < len) return Result.failure(IllegalArgumentException("Invalid length or buffer size"))

        val available = size - readPos
        return when {
            available <= 0 -> Result.success(0) // EOF
            else -> {
                val toCopy = minOf(len, available)
                val endPos = readPos + toCopy
                buffer.copyInto(buf, offset, readPos, endPos)
                readPos = endPos
                Result.success(toCopy)
            }
        }
    }

    override fun readToEnd(buf: MutableList<UByte>, offset: Int): Result<Int> {
        val available = size - readPos
        if (available <= 0) return Result.success(0)

        buf.addAll(offset, List(available) { buffer[readPos + it].toUByte() })
        readPos = size
        return Result.success(available)
    }

    override fun write(buf: ByteArray, offset: Int, len: Int): Result<Int> {
        if (len < 0 || buf.size < len) return Result.failure(IllegalArgumentException("Invalid length or buffer size"))

        ensureCapacity(writePos + len)
        buf.copyInto(buffer, writePos, offset, len)
        writePos += len
        size = max(size, writePos)
        return Result.success(len)
    }

    override fun writeAll(buf: ByteArray, offset: Int): Result<Unit> {
        ensureCapacity(writePos + buf.size)
        buf.copyInto(buffer, writePos, offset)
        writePos += buf.size
        size = max(size, writePos)
        return Result.success(Unit)
    }

    override fun flush(): Result<Unit> = Result.success(Unit)

    // 内存管理方法
    fun resetRead() {
        readPos = 0
    }

    fun clear() {
        buffer = ByteArray(buffer.size.coerceAtLeast(INITIAL_SIZE)) // 清空时保留扩容后的大小
        readPos = 0
        writePos = 0
        size = 0
    }

    private fun ensureCapacity(required: Int) {
        if (required > buffer.size) {
            val newSize = max(buffer.size * 2, required)
            buffer = buffer.copyOf(newSize)
        }
    }

    // 调试方法
    fun snapshot(): ByteArray = buffer.copyOfRange(0, size)

    companion object {
        const val INITIAL_SIZE = 16
    }
}
