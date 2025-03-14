package io

interface Write {
    fun write(buf: ByteArray, len: Int): Result<Int>
    fun writeAll(buf: ByteArray): Result<Unit>
    fun flush(): Result<Unit>
}
