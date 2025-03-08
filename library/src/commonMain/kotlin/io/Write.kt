package io

interface Write {
    fun write(buf: ByteArray, len: UInt): Result<UInt>
    fun write_all(buf: ByteArray): Result<Unit>
    fun flush(): Result<Unit>
}
