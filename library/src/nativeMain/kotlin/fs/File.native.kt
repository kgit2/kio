package fs

import io.Read
import io.Write
import path.Path

actual class File actual constructor(path: Path) : Read, Write {
    override fun read(buf: ByteArray, len: Int): Result<Int> {
        TODO("Not yet implemented")
    }

    override fun readToEnd(buf: MutableList<UByte>): Result<Int> {
        TODO("Not yet implemented")
    }

    override fun write(buf: ByteArray, len: Int): Result<Int> {
        TODO("Not yet implemented")
    }

    override fun writeAll(buf: ByteArray): Result<Unit> {
        TODO("Not yet implemented")
    }

    override fun flush(): Result<Unit> {
        TODO("Not yet implemented")
    }
}
