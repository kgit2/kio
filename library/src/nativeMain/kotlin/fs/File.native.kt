package fs

import io.Read
import io.Write
import io.buffered.SlicedByteArray
import path.Path

actual class File actual constructor(path: Path) : Read, Write {
    override fun read(buf: SlicedByteArray, offset: Int, len: Int): Int {
        TODO("Not yet implemented")
    }

    override fun readToEnd(buf: MutableList<UByte>, offset: Int): Int {
        TODO("Not yet implemented")
    }

    override fun write(buf: SlicedByteArray, offset: Int, len: Int): Int {
        TODO("Not yet implemented")
    }

    override fun writeAll(buf: SlicedByteArray, offset: Int) {
        TODO("Not yet implemented")
    }

    override fun flush() {
        TODO("Not yet implemented")
    }
}
