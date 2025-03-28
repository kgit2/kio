package io

interface Read {
    fun read(buf: ByteArray, offset: Int, len: Int): Result<Int>
    fun readToEnd(buf: MutableList<UByte>, offset: Int): Result<Int>
}
