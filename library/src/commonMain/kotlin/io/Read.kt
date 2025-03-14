package io

interface Read {
    fun read(buf: ByteArray, len: Int): Result<Int>
    fun readToEnd(buf: MutableList<UByte>): Result<Int>
}
