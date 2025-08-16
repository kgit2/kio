package io

actual object Stdout : Write {
    override fun write(buf: ByteArray, len: UInt): Result<UInt> {
        TODO("Not yet implemented")
    }

    override fun flush() {
        TODO("Not yet implemented")
    }
}
