package io

class BufferedReader<T : Read>(
    private val source: T,
    private val bufferSize: Int = 8192,
) : Read {
    private val buffer = ByteArray(bufferSize)
    private var readPos = 0
    private var writePos = 0
    private var eof = false    // 是否已经到达 EOF

    override fun read(buf: ByteArray, offset: Int, len: Int): Result<Int> {
        if (len < 0 || buf.size < len) {
            return Result.failure(IllegalArgumentException("Invalid length or buffer size"))
        }
        if (len == 0) return Result.success(0)

        var bufOffset = offset
        var bufNeed = len
        var available = writePos - readPos

        while (true) {
            if (bufNeed <= available) {
                buffer.copyInto(buf, bufOffset, readPos, readPos + bufNeed)
                readPos += bufNeed
                break
            } else {
                val toRead = bufNeed - available
                val result = source.read(buffer, writePos, toRead)
                if (result.isFailure) return result
                val bytesRead = result.getOrThrow()
                if (bytesRead == 0) {
                    eof = true
                    break
                }
                writePos += bytesRead
                available += bytesRead
            }
        }

    }

    override fun readToEnd(buf: MutableList<UByte>, offset: Int): Result<Int> {
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
