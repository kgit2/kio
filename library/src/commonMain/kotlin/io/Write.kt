package io

interface Write {
    fun write(buf: ByteArray, offset: Int, len: Int): Result<Int>
    fun writeAll(buf: ByteArray, offset: Int): Result<Unit>
    fun flush(): Result<Unit>
}
