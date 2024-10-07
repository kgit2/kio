package io.write

interface Write {
    fun write(buf: String): ULong

    fun flush()
}
