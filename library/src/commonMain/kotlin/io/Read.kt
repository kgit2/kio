package io

interface Read {
    fun read(buf: ByteArray, len: UInt): Result<UInt>
    fun read_to_end(buf: MutableList<UByte>): Result<UInt>
}
