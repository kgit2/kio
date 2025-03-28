package fs

import io.Read
import io.Write
import path.Path

actual class File actual constructor(path: Path) : Read, Write {
    override fun read(buf: ByteArray, len: Int): Result<Int> {
        TODO("Not yet implemented")
    }

    override fun readToEnd(buf: MutableList<UByte>, offset: Int): Result<Int> {
        TODO("Not yet implemented")
    }

    override fun write(buf: ByteArray, offset: Int, len: Int): Result<Int> {
        TODO("Not yet implemented")
    }

    override fun writeAll(buf: ByteArray, offset: Int): Result<Unit> {
        TODO("Not yet implemented")
    }

    override fun flush(): Result<Unit> {
        TODO("Not yet implemented")
    }
}
