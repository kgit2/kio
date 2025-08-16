package io.buffered

class SlicedByteArray private constructor(
    private val buffer: ByteArray,
    val offset: Int,
    val size: Int
) : Iterable<Byte> {
    operator fun get(index: Int): Byte {
        checkIndex(index)
        return buffer[offset + index]
    }

    fun unsafeGet(index: Int): Byte = buffer[offset + index]

    operator fun set(index: Int, value: Byte) {
        checkIndex(index)
        buffer[offset + index] = value
    }

    fun unsafeSet(index: Int, value: Byte) {
        buffer[offset + index] = value
    }

    fun isEmpty(): Boolean = size == 0

    fun slice(start: Int, length: Int): SlicedByteArray {
        if (start < 0 || length < 0 || start + length > size)
            throw IndexOutOfBoundsException("Invalid subArray")
        return SlicedByteArray(buffer, offset + start, length)
    }

    fun slice(range: IntRange): SlicedByteArray = slice(range.first, range.last - range.first + 1)

    fun toByteArray(): ByteArray = buffer.copyOfRange(offset, offset + size)

    fun copyInto(
        destination: SlicedByteArray,
        destinationOffset: Int = 0,
        startIndex: Int = 0,
        endIndex: Int = size
    ) {
        if (startIndex < 0 || endIndex > size || startIndex > endIndex)
            throw IndexOutOfBoundsException("Invalid copy size")
        buffer.copyInto(destination.buffer, destinationOffset + destination.offset, offset + startIndex, offset + endIndex)
    }

    fun copy(): SlicedByteArray {
        return SlicedByteArray(buffer.copyOf(), 0, size)
    }

    fun copyOf(length: Int): SlicedByteArray {
        if (length !in 0..size)
            throw IndexOutOfBoundsException("Invalid copy size")
        return SlicedByteArray(buffer.copyOf(length), 0, length)
    }

    fun copyOfRange(start: Int, end: Int): SlicedByteArray {
        if (start < 0 || end > size || start > end)
            throw IndexOutOfBoundsException("Invalid copy range")
        return SlicedByteArray(buffer.copyOfRange(offset + start, offset + end), 0, end - start)
    }

    override operator fun iterator(): ByteIterator =
        object : ByteIterator() {
            private var index = 0
            override fun hasNext() = index < size
            override fun nextByte() = get(index++)
        }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is SlicedByteArray || size != other.size) return false
        for (i in 0 until size) if (this[i] != other[i]) return false
        return true
    }

    override fun hashCode(): Int {
        var result = 1
        for (i in 0 until size) result = 31 * result + this[i]
        return result
    }

    override fun toString(): String =
        joinToString(prefix = "[", postfix = "]")

    fun joinToString(
        separator: String = ", ",
        prefix: String = "",
        postfix: String = "",
        transform: (Byte) -> String = { it.toString() }
    ): String {
        return buildString {
            append(prefix)
            forEachIndexed { i, b ->
                if (i > 0) append(separator)
                append(transform(b.code.toByte()))
            }
            append(postfix)
        }
    }

    inline fun forEachIndexed(action: (Int, Byte) -> Unit) {
        for (i in 0 until size) action(i, this[i])
    }

    private fun checkIndex(index: Int) {
        if (index !in 0 until size)
            throw IndexOutOfBoundsException("Index $index out of bounds for size $size")
    }

    @OptIn(ExperimentalUnsignedTypes::class)
    fun asUByteArray(): UByteArray {
        return buffer.asUByteArray()
    }

    companion object {
        fun wrap(array: ByteArray): SlicedByteArray =
            SlicedByteArray(array, 0, array.size)

        fun allocate(size: Int): SlicedByteArray =
            SlicedByteArray(ByteArray(size), 0, size)
    }
}
