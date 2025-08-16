package io

import io.buffered.SlicedByteArray

interface Write {
    fun write(buf: SlicedByteArray, offset: Int, len: Int): Int
    fun writeAll(buf: SlicedByteArray, offset: Int)
    fun flush()
}
