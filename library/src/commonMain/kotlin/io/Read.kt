package io

import io.buffered.SlicedByteArray

interface Read {
    fun read(buf: SlicedByteArray, offset: Int, len: Int): Int
    fun readToEnd(buf: MutableList<UByte>, offset: Int): Int
}
