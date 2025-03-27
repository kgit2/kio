package memory

import io.Read
import io.Write
import kotlin.math.max

class MemoryBuffer : Read, Write {
    // 内部存储使用 ByteArray 实现高效访问
    private var buffer = ByteArray(16)
    private var readPos = 0
    private var writePos = 0
    private var size = 0  // 当前有效数据长度

    override fun read(buf: ByteArray, len: Int): Result<Int> {
        if (len < 0 || buf.size < len) return Result.failure(IllegalArgumentException("Invalid length or buffer size"))

        val available = size - readPos
        return when {
            available <= 0 -> Result.success(0) // EOF
            else -> {
                val bytesToRead = minOf(len, available)
                buffer.copyInto(buf, 0, readPos, readPos + bytesToRead)
                readPos += bytesToRead
                Result.success(bytesToRead)
            }
        }
    }

    override fun readToEnd(buf: MutableList<UByte>): Result<Int> {
        val available = size - readPos
        if (available <= 0) return Result.success(0)

        (readPos until size).forEach {
            buf.add(buffer[it].toUByte())
        }
        readPos = size
        return Result.success(available)
    }

    override fun write(buf: ByteArray, len: Int): Result<Int> {
        if (len < 0 || buf.size < len) return Result.failure(IllegalArgumentException("Invalid length or buffer size"))

        ensureCapacity(writePos + len)
        buf.copyInto(buffer, writePos, 0, len)
        writePos += len
        size = max(size, writePos)
        return Result.success(len)
    }

    override fun writeAll(buf: ByteArray): Result<Unit> {
        ensureCapacity(writePos + buf.size)
        buf.copyInto(buffer, writePos)
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
        buffer = ByteArray(buffer.size.coerceAtLeast(16)) // 清空时保留扩容后的大小
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
}
