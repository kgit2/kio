package io

import io.buffered.SlicedByteArray

interface BufferedRead : Read {
    fun fillBuf(): SlicedByteArray
    fun consume(amount: Int)
    fun hasDataLeft(): Boolean
    fun readUntil(byte: Byte, buf: SlicedByteArray, offset: Int, len: Int): Boolean
    fun skipUntil(byte: Byte): Int
    fun readLine(): String
    fun split(byte: Byte): Sequence<BufferedRead>
    fun lines(): Sequence<BufferedRead>
}
