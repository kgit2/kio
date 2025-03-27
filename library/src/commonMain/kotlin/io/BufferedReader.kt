package io

class BufferedReader<T: Read>(
    private val source: T,
    private val bufferSize: Int = 8192
) : Read {
    private val buffer = ByteArray(bufferSize)
    private var fillPos = 0    // 下一个未读字节的位置
    private var limit = 0      // buffer 中当前有效数据的末尾位置
    private var eof = false    // 是否已经到达 EOF

    override fun read(buf: ByteArray, len: Int): Result<Int> {
        if (len < 0 || buf.size < len) {
            return Result.failure(IllegalArgumentException("Invalid length or buffer size"))
        }
        if (len == 0) return Result.success(0)
        var bytesRead = 0

        // 1. 优先从内部缓冲区读取数据
        if (fillPos < limit) {
            val toCopy = minOf(len, limit - fillPos)
            buffer.copyInto(buf, 0, fillPos, fillPos + toCopy)
            fillPos += toCopy
            bytesRead += toCopy
            if (bytesRead >= len) return Result.success(bytesRead)
        }

        // 2. 如果内部缓冲区已空，尝试从 source 填充新数据
        while (bytesRead < len) {
            if (eof) return Result.success(bytesRead)

            // 2.1 先将数据读取到内部缓冲区
            fillPos = 0
            val result = source.read(buffer, buffer.size) // 调用 source.read(buf, len)
            if (result.isFailure) return result
            val n = result.getOrThrow()

            if (n == 0) {
                eof = true
                limit = 0 // 关键修复：重置 limit 避免后续误判
                return Result.success(bytesRead)
            }
            limit = n

            // 2.2 从内部缓冲区复制到用户缓冲区
            val remaining = len - bytesRead
            val toCopy = minOf(remaining, limit - fillPos)
            buffer.copyInto(buf, bytesRead, fillPos, fillPos + toCopy)
            fillPos += toCopy
            bytesRead += toCopy
        }

        return Result.success(bytesRead)
    }

    override fun readToEnd(buf: MutableList<UByte>): Result<Int> {
        var totalBytes = 0

        // 从缓冲区先消耗所有剩余数据
        while (fillPos < limit) {
            buf.add(buffer[fillPos].toUByte())
            fillPos++
            totalBytes++
        }

        // 继续从底层读取直到 EOF
        while (!eof) {
            val chunk = ByteArray(bufferSize)
            val result = source.read(chunk, chunk.size)
            if (result.isFailure) return result
            val bytesRead = result.getOrThrow()
            if (bytesRead == 0) {
                eof = true
                break
            }
            totalBytes += bytesRead
            buf.addAll(chunk.take(bytesRead).map { it.toUByte() })
        }

        return Result.success(totalBytes)
    }

    // 继续保留之前的 readLine 方法
    fun readLine(): String? {
        val lineBuffer = StringBuilder()

        while (true) {
            if (fillPos >= limit) {
                // 缓冲区空了，需要填充
                fillPos = 0
                limit = 0
                val result = source.read(buffer, buffer.size)
                if (result.isFailure) return null // 处理底层错误
                val bytesRead = result.getOrThrow()
                if (bytesRead <= 0) return if (lineBuffer.isEmpty()) null else lineBuffer.toString()
                limit = bytesRead
            }

            // 从缓冲区中读取一行内容
            while (fillPos < limit) {
                val byte = buffer[fillPos++]
                if (byte.toInt().toChar() == '\n') {
                    return lineBuffer.toString()
                } else if (byte.toInt().toChar() == '\r') {
                    // 检查是否是 \r\n 的组合
                    if (fillPos < limit && buffer[fillPos] == '\n'.code.toByte()) {
                        fillPos++
                    }
                    return lineBuffer.toString()
                }
                lineBuffer.append(byte.toInt().toChar())
            }
        }
    }
}
